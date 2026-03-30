//! Global database state: pragmas, indexes, and the lock that guards them.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Mutex, MutexGuard};

use crate::vdbe::exec::OrdRegister;

// ── PragmaState ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PragmaState {
    pub cache_size: i64,
    pub page_size: i64,
    pub auto_vacuum: i64,
    pub user_version: i64,
    pub application_id: i64,
    pub synchronous: i64,
    pub query_only: bool,
    pub journal_mode: JournalMode,
}

impl Default for PragmaState {
    fn default() -> Self {
        PragmaState {
            cache_size: 2000,
            page_size: 4096,
            auto_vacuum: 0,
            user_version: 0,
            application_id: 0,
            synchronous: 2,
            query_only: false,
            journal_mode: JournalMode::Delete,
        }
    }
}

// ── JournalMode ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalMode {
    Delete,
    Truncate,
    Persist,
    Memory,
    Wal,
    Off,
}

// ── IndexData ─────────────────────────────────────────────────────────────────

/// A secondary index: maps key tuples to lists of matching rowids.
#[derive(Debug, Clone, Default)]
pub struct IndexData {
    /// Name of the table this index covers.
    pub table: String,
    /// Column indices (0-based) that form the index key.
    pub columns: Vec<usize>,
    /// Actual index data: key tuple → list of matching rowids.
    pub entries: BTreeMap<Vec<OrdRegister>, Vec<i64>>,
}

// ── GlobalTableEntry ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct GlobalTableEntry {
    pub indexes: HashMap<String, IndexData>,
}

// ── GlobalState ───────────────────────────────────────────────────────────────

pub struct GlobalState {
    pub tables: HashMap<String, GlobalTableEntry>,
    pub pragmas: PragmaState,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            tables: HashMap::new(),
            pragmas: PragmaState::default(),
        }
    }
}

// ── Global lock ───────────────────────────────────────────────────────────────

static GLOBAL: Mutex<Option<GlobalState>> = Mutex::new(None);

pub fn lock_global() -> impl std::ops::DerefMut<Target = GlobalState> {
    struct Guard(MutexGuard<'static, Option<GlobalState>>);
    impl std::ops::Deref for Guard {
        type Target = GlobalState;
        fn deref(&self) -> &GlobalState {
            self.0.as_ref().unwrap()
        }
    }
    impl std::ops::DerefMut for Guard {
        fn deref_mut(&mut self) -> &mut GlobalState {
            self.0.as_mut().unwrap()
        }
    }

    let mut guard = GLOBAL.lock().expect("wip_db global lock poisoned");
    if guard.is_none() {
        *guard = Some(GlobalState::default());
    }
    // SAFETY: the Mutex is 'static so the guard is also 'static-lifetime.
    let guard: MutexGuard<'static, Option<GlobalState>> =
        unsafe { std::mem::transmute(guard) };
    Guard(guard)
}
