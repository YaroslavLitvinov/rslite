//! Virtual Database Engine — opcodes, bytecode programs, and compiler.
//!
//! Pipeline: `Stmt` (AST) → `compile()` → `Program` (bytecode).

pub mod exec;
pub mod record;
pub mod storage;
pub mod btree_storage;

mod opcodes;
mod compiler;
mod error;

// Re-export public API
pub use opcodes::{Opcode, P4, Instr, Program};
pub use error::CompileError;
pub use compiler::compile;
pub use storage::{StorageBackend, TableCursor, MemStorage, MemCursor};
pub use btree_storage::{BtreeStorage, BtreeCursorWrapper};

use crate::schema::Schema;
use std::collections::BTreeMap;
use crate::vdbe::exec::MemTable;
pub struct PersistentStore {
    pub tables: BTreeMap<String, MemTable>,
    pub schema: Schema,
}
