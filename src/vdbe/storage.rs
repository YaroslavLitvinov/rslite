//! Storage abstraction for the VDBE executor.
//!
//! Two implementations are provided:
//!   - `MemStorage`   — in-memory BTreeMap (default, zero setup required)
//!   - `BtreeStorage` — SQLite pager/B-tree (see btree_storage.rs)
//!
//! The `StorageBackend` trait is the integration point: swap in either
//! implementation when constructing a `Vm`.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::exec::{MemTable, Register};

// ── Error codes (mirrors sqlite3.h) ──────────────────────────────────────────

pub const SQLITE_OK: i32 = 0;
pub const SQLITE_ERROR: i32 = 1;
pub const SQLITE_CORRUPT: i32 = 11;
pub const SQLITE_FULL: i32 = 13;
pub const SQLITE_ROW: i32 = 100;
pub const SQLITE_DONE: i32 = 101;

// ── Core traits ───────────────────────────────────────────────────────────────

/// A positioned cursor into a single table.
///
/// All methods operate on the "current row" maintained internally.
/// Callers must call `rewind()` or `seek_rowid()` before reading.
pub trait TableCursor {
    /// Position at the first row.  Returns `true` if the table is empty.
    fn rewind(&mut self) -> Result<bool, i32>;
    /// Advance to the next row.  Returns `true` if another row exists.
    fn next(&mut self) -> Result<bool, i32>;
    /// Move to the previous row.  Returns `true` if a previous row exists.
    fn prev(&mut self) -> Result<bool, i32>;
    /// Read column `col` (0-based) from the current row.
    fn column(&mut self, col: usize) -> Result<Register, i32>;
    /// Integer rowid of the current row.
    fn rowid(&mut self) -> Result<i64, i32>;
    /// Insert a row; positions cursor at the new row.
    fn insert(&mut self, rowid: i64, record: &[Register]) -> Result<(), i32>;
    /// Delete the current row.
    fn delete(&mut self) -> Result<(), i32>;
    /// Return the next available rowid (max existing + 1, or 1 if empty).
    fn alloc_rowid(&mut self) -> Result<i64, i32>;
    /// Seek to `rowid`.  Returns `true` if found; positions cursor there.
    fn seek_rowid(&mut self, rowid: i64) -> Result<bool, i32>;
    /// Name of the underlying table (used by index-scan opcodes).
    fn table_name(&self) -> &str;
}

/// Factory for cursors plus table-level operations.
///
/// Must be `Clone` so sub-programs can share (or snapshot) the same storage.
pub trait StorageBackend: Clone {
    type Cursor: TableCursor;

    fn open_cursor(&mut self, table_name: &str, writable: bool) -> Result<Self::Cursor, i32>;
    fn drop_table(&mut self, table_name: &str) -> Result<(), i32>;
}

// ── MemStorage ────────────────────────────────────────────────────────────────
//
// Tables are heap-allocated behind Rc<RefCell<…>> so that multiple cursors
// can point into the same table without lifetime entanglement.

type MemTableRef = Rc<RefCell<MemTable>>;

/// In-memory storage backed by BTreeMap — no setup, no persistence.
#[derive(Clone, Default)]
pub struct MemStorage {
    tables: BTreeMap<String, MemTableRef>,
}

impl MemStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Seed the store with a pre-built MemTable (e.g. from a previous session).
    pub fn import(&mut self, name: impl Into<String>, data: MemTable) {
        self.tables
            .insert(name.into(), Rc::new(RefCell::new(data)));
    }

    /// Access the raw MemTable ref (for debugging / testing).
    pub fn get_table(&self, name: &str) -> Option<MemTableRef> {
        self.tables.get(name).cloned()
    }

    /// Snapshot all tables as owned MemTable values (clones the underlying data).
    pub fn snapshot_tables(&self) -> BTreeMap<String, MemTable> {
        self.tables
            .iter()
            .map(|(k, v)| (k.clone(), v.borrow().clone()))
            .collect()
    }

    /// Merge mutations from another MemStorage instance back into this one.
    ///
    /// Tables present in `other` overwrite the corresponding tables here.
    /// Tables present only in `self` are left untouched.
    pub fn merge(&mut self, other: MemStorage) {
        for (name, tbl_ref) in other.tables {
            let tbl = Rc::try_unwrap(tbl_ref)
                .map(|cell| cell.into_inner())
                .unwrap_or_else(|rc| rc.borrow().clone());
            self.tables.insert(name, Rc::new(RefCell::new(tbl)));
        }
    }
}

impl StorageBackend for MemStorage {
    type Cursor = MemCursor;

    fn open_cursor(&mut self, table_name: &str, _writable: bool) -> Result<MemCursor, i32> {
        let tbl = self
            .tables
            .entry(table_name.to_string())
            .or_insert_with(|| Rc::new(RefCell::new(MemTable::new())))
            .clone();
        Ok(MemCursor {
            name: table_name.to_string(),
            tbl,
            current_rowid: None,
        })
    }

    fn drop_table(&mut self, table_name: &str) -> Result<(), i32> {
        self.tables.remove(table_name);
        Ok(())
    }
}

// ── MemCursor ─────────────────────────────────────────────────────────────────

pub struct MemCursor {
    name: String,
    tbl: MemTableRef,
    current_rowid: Option<i64>,
}

impl TableCursor for MemCursor {
    fn rewind(&mut self) -> Result<bool, i32> {
        let t = self.tbl.borrow();
        let first = t.rows.keys().next().copied();
        drop(t);
        self.current_rowid = first;
        Ok(first.is_none())
    }

    fn next(&mut self) -> Result<bool, i32> {
        let t = self.tbl.borrow();
        let next = self
            .current_rowid
            .and_then(|rid| t.rows.range((rid + 1)..).next().map(|(&k, _)| k));
        drop(t);
        self.current_rowid = next;
        Ok(next.is_some())
    }

    fn prev(&mut self) -> Result<bool, i32> {
        let t = self.tbl.borrow();
        let prev = self
            .current_rowid
            .and_then(|rid| t.rows.range(..rid).next_back().map(|(&k, _)| k));
        drop(t);
        self.current_rowid = prev;
        Ok(prev.is_some())
    }

    fn column(&mut self, col: usize) -> Result<Register, i32> {
        let t = self.tbl.borrow();
        Ok(self
            .current_rowid
            .and_then(|rid| t.rows.get(&rid))
            .and_then(|row| row.get(col))
            .cloned()
            .unwrap_or(Register::Null))
    }

    fn rowid(&mut self) -> Result<i64, i32> {
        self.current_rowid.ok_or(SQLITE_ERROR)
    }

    fn insert(&mut self, rowid: i64, record: &[Register]) -> Result<(), i32> {
        let mut t = self.tbl.borrow_mut();
        t.rows.insert(rowid, record.to_vec());
        if rowid >= t.next_rowid {
            t.next_rowid = rowid + 1;
        }
        drop(t);
        self.current_rowid = Some(rowid);
        Ok(())
    }

    fn delete(&mut self) -> Result<(), i32> {
        let rowid = self.current_rowid.ok_or(SQLITE_ERROR)?;
        self.tbl.borrow_mut().rows.remove(&rowid);
        // Leave current_rowid pointing at the deleted key.
        // `next()` will call range((rowid+1)..) which skips past it naturally.
        Ok(())
    }

    fn alloc_rowid(&mut self) -> Result<i64, i32> {
        let mut t = self.tbl.borrow_mut();
        let id = if t.next_rowid > 0 {
            t.next_rowid
        } else {
            t.rows.keys().next_back().copied().unwrap_or(0) + 1
        };
        t.next_rowid = id + 1;
        Ok(id)
    }

    fn seek_rowid(&mut self, rowid: i64) -> Result<bool, i32> {
        let t = self.tbl.borrow();
        let found = t.rows.contains_key(&rowid);
        drop(t);
        if found {
            self.current_rowid = Some(rowid);
        }
        Ok(found)
    }

    fn table_name(&self) -> &str {
        &self.name
    }
}
