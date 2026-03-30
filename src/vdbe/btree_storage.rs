//! B-tree storage backend — wires the porting VDBE to SQLite's real pager/btree.
//!
//! # Safety
//! All functions here are `unsafe` by construction: they call into the
//! C2Rust-transpiled btree layer via raw pointers.  The caller (Vm) must
//! guarantee:
//!   - The `*mut sqlite3` handle is alive for the lifetime of this storage.
//!   - A write transaction is open before any mutating cursor operation.
//!   - Cursors are closed (dropped) before the Btree is closed.
//!
//! # Integration
//! 1. Open a SQLite database via `sqlite3_open` (C API or existing main-crate
//!    function).
//! 2. Build a `root_pages` map from table-name → root-page-number by querying
//!    `sqlite_master` (pgnoRoot field of each Table in the schema).
//! 3. Construct `BtreeStorage::new(db, root_pages)`.
//! 4. Pass as backend to `Vm::new(…, BtreeStorage)`.

use std::alloc;
use std::collections::HashMap;

use super::record::{decode_column, encode_record};
use super::storage::{StorageBackend, TableCursor, SQLITE_ERROR, SQLITE_FULL, SQLITE_OK};
use super::exec::Register;

// Re-export btree types from the main crate.
use crate::src::src::btree::{
    sqlite3BtreeCursor, sqlite3BtreeCursorSize, sqlite3BtreeCloseCursor, sqlite3BtreeDelete,
    sqlite3BtreeFirst, sqlite3BtreeLast, sqlite3BtreeInsert, sqlite3BtreeIntegerKey,
    sqlite3BtreeNext, sqlite3BtreePayload, sqlite3BtreePayloadSize, sqlite3BtreePrevious,
    sqlite3BtreeTableMoveto, BtreePayload,
};
use crate::src::headers::btreeInt_h::BtCursor;
use crate::src::headers::sqliteInt_h::sqlite3;

// ── BtreeStorage ─────────────────────────────────────────────────────────────

/// Storage backend backed by SQLite's B-tree/pager layer.
pub struct BtreeStorage {
    /// Raw database handle — must outlive this struct.
    db: *mut sqlite3,
    /// table name → root page number (from sqlite_master).
    root_pages: HashMap<String, u32>,
}

// SAFETY: BtreeStorage is only ever used on one thread at a time in the
// current porting codebase (no async, no cross-thread sharing).
unsafe impl Send for BtreeStorage {}

impl Clone for BtreeStorage {
    /// Cloning shares the same database pointer and duplicates the page map.
    /// The clone is intended for sub-program Vms that see the same live data.
    fn clone(&self) -> Self {
        BtreeStorage {
            db: self.db,
            root_pages: self.root_pages.clone(),
        }
    }
}

impl BtreeStorage {
    /// Construct from an open database handle and a pre-built page map.
    ///
    /// # Safety
    /// `db` must be a valid, open `*mut sqlite3` that remains alive for the
    /// entire lifetime of this `BtreeStorage` and all cursors it produces.
    pub unsafe fn new(db: *mut sqlite3, root_pages: HashMap<String, u32>) -> Self {
        BtreeStorage { db, root_pages }
    }

    /// Return the main-database Btree handle from the sqlite3 struct.
    fn main_btree(&self) -> *mut crate::src::headers::btreeInt_h::Btree {
        // aDb[0] is always the main database.
        unsafe { (*(*self.db).aDb.offset(0)).pBt }
    }
}

impl StorageBackend for BtreeStorage {
    type Cursor = BtreeCursorWrapper;

    fn open_cursor(&mut self, table_name: &str, writable: bool) -> Result<BtreeCursorWrapper, i32> {
        let root_page = *self
            .root_pages
            .get(table_name)
            .ok_or(SQLITE_ERROR)?;

        // BtCursor must be allocated with the size that the btree layer
        // reports (it may differ from sizeof(BtCursor) after padding).
        let cursor_size = unsafe { sqlite3BtreeCursorSize() } as usize;
        let layout = alloc::Layout::from_size_align(cursor_size, 8).unwrap();
        let raw = unsafe { alloc::alloc_zeroed(layout) } as *mut BtCursor;
        if raw.is_null() {
            return Err(crate::src::headers::sqlite3_h::SQLITE_NOMEM);
        }

        let wr_flag = writable as i32;
        let rc = unsafe {
            sqlite3BtreeCursor(
                self.main_btree(),
                root_page,
                wr_flag,
                std::ptr::null_mut(), // pKeyInfo — NULL for integer-key tables
                raw,
            )
        };
        if rc != SQLITE_OK {
            unsafe { alloc::dealloc(raw as *mut u8, layout) };
            return Err(rc);
        }

        Ok(BtreeCursorWrapper {
            name: table_name.to_string(),
            raw,
            layout,
            valid: false,
        })
    }

    fn drop_table(&mut self, table_name: &str) -> Result<(), i32> {
        // Remove from the local page map; the actual B-tree page reclamation
        // requires sqlite3BtreeDropTable which needs a write transaction and
        // schema update — left as a TODO until the schema layer is wired in.
        self.root_pages.remove(table_name);
        Ok(())
    }
}

// ── BtreeCursorWrapper ────────────────────────────────────────────────────────

/// Owns a heap-allocated `BtCursor` and closes it on drop.
pub struct BtreeCursorWrapper {
    name: String,
    raw: *mut BtCursor,
    layout: alloc::Layout,
    /// False until the cursor has been positioned (rewind/seek).
    valid: bool,
}

// SAFETY: see BtreeStorage comment above.
unsafe impl Send for BtreeCursorWrapper {}

impl Drop for BtreeCursorWrapper {
    fn drop(&mut self) {
        unsafe {
            sqlite3BtreeCloseCursor(self.raw);
            alloc::dealloc(self.raw as *mut u8, self.layout);
        }
    }
}

impl TableCursor for BtreeCursorWrapper {
    fn rewind(&mut self) -> Result<bool, i32> {
        let mut empty: i32 = 0;
        let rc = unsafe { sqlite3BtreeFirst(self.raw, &mut empty) };
        if rc != SQLITE_OK {
            return Err(rc);
        }
        self.valid = empty == 0;
        Ok(empty != 0) // true = table is empty
    }

    fn next(&mut self) -> Result<bool, i32> {
        if !self.valid {
            return Ok(false);
        }
        let rc = unsafe { sqlite3BtreeNext(self.raw, 0) };
        match rc {
            // SQLITE_OK → more rows; SQLITE_DONE (101) → end of table
            0 => {
                self.valid = true;
                Ok(true)
            }
            101 => {
                self.valid = false;
                Ok(false)
            }
            _ => Err(rc),
        }
    }

    fn prev(&mut self) -> Result<bool, i32> {
        if !self.valid {
            return Ok(false);
        }
        let rc = unsafe { sqlite3BtreePrevious(self.raw, 0) };
        match rc {
            0 => {
                self.valid = true;
                Ok(true)
            }
            101 => {
                self.valid = false;
                Ok(false)
            }
            _ => Err(rc),
        }
    }

    fn column(&mut self, col: usize) -> Result<Register, i32> {
        if !self.valid {
            return Ok(Register::Null);
        }
        let size = unsafe { sqlite3BtreePayloadSize(self.raw) };
        let mut buf = vec![0u8; size as usize];
        let rc = unsafe {
            sqlite3BtreePayload(self.raw, 0, size, buf.as_mut_ptr() as *mut _)
        };
        if rc != SQLITE_OK {
            return Err(rc);
        }
        decode_column(&buf, col).map_err(|_| crate::src::headers::sqlite3_h::SQLITE_CORRUPT)
    }

    fn rowid(&mut self) -> Result<i64, i32> {
        if !self.valid {
            return Err(SQLITE_ERROR);
        }
        Ok(unsafe { sqlite3BtreeIntegerKey(self.raw) })
    }

    fn insert(&mut self, rowid: i64, record: &[Register]) -> Result<(), i32> {
        let data = encode_record(record);
        let payload = BtreePayload {
            pKey: std::ptr::null(),
            nKey: rowid,
            pData: data.as_ptr() as *const _,
            aMem: std::ptr::null_mut(),
            nMem: 0,
            nData: data.len() as i32,
            nZero: 0,
        };
        let rc = unsafe { sqlite3BtreeInsert(self.raw, &payload, 0, 0) };
        if rc != SQLITE_OK {
            return Err(rc);
        }
        self.valid = true;
        Ok(())
    }

    fn delete(&mut self) -> Result<(), i32> {
        if !self.valid {
            return Err(SQLITE_ERROR);
        }
        let rc = unsafe { sqlite3BtreeDelete(self.raw, 0) };
        if rc != SQLITE_OK {
            return Err(rc);
        }
        self.valid = false;
        Ok(())
    }

    fn alloc_rowid(&mut self) -> Result<i64, i32> {
        // Find the current maximum rowid and add one.
        let mut last_empty: i32 = 0;
        let rc = unsafe { sqlite3BtreeLast(self.raw, &mut last_empty) };
        if rc != SQLITE_OK {
            return Err(rc);
        }
        if last_empty != 0 {
            // Table is empty — restore cursor state.
            self.valid = false;
            return Ok(1);
        }
        let last = unsafe { sqlite3BtreeIntegerKey(self.raw) };
        if last == i64::MAX {
            return Err(SQLITE_FULL);
        }
        // alloc_rowid must not move the cursor permanently — callers expect to
        // be able to continue scanning after calling it (e.g. NewRowid opcode).
        // Restore to invalid so the next rewind/seek re-positions.
        self.valid = false;
        Ok(last + 1)
    }

    fn seek_rowid(&mut self, rowid: i64) -> Result<bool, i32> {
        let mut res: i32 = 0;
        let rc = unsafe { sqlite3BtreeTableMoveto(self.raw, rowid, 0, &mut res) };
        if rc != SQLITE_OK {
            return Err(rc);
        }
        // res == 0 means exact match found.
        self.valid = res == 0;
        Ok(res == 0)
    }

    fn table_name(&self) -> &str {
        &self.name
    }
}
