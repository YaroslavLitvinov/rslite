//! High-level Rust database interface.
//!
//! Wires the SQL parser, VDBE compiler, and execution engine together into a
//! single `Database` struct. Uses `MemStorage` by default; swap in
//! `BtreeStorage` to run against the real pager/B-tree layer.
//!
//! # Example
//! ```ignore
//! let mut db = Database::new();
//! db.create_table("CREATE TABLE t (id INTEGER, name TEXT)");
//! let rows = db.query("SELECT * FROM t WHERE id = 1").unwrap();
//! ```

use std::collections::BTreeMap;

use crate::schema::{Column, Schema, Table};
use crate::sql::parser::parse_sql;
use crate::sql::ast::{Stmt, CreateTableBody, ColumnConstraint};
use crate::vdbe::compile;
use crate::vdbe::exec::{MemTable, Register, Vm, step};
use crate::vdbe::storage::{MemStorage, SQLITE_ROW, SQLITE_DONE};
use crate::wip_db::{IndexData, PragmaState};

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum DbError {
    Parse(String),
    Compile(String),
    Exec(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Parse(m)   => write!(f, "parse error: {m}"),
            DbError::Compile(m) => write!(f, "compile error: {m}"),
            DbError::Exec(m)    => write!(f, "exec error: {m}"),
        }
    }
}
impl std::error::Error for DbError {}

// ── Database ──────────────────────────────────────────────────────────────────

/// An in-memory SQLite-compatible database backed by the pure-Rust VDBE.
pub struct Database {
    schema:  Schema,
    storage: MemStorage,
    pragmas: PragmaState,
    indexes: BTreeMap<String, IndexData>,
}

impl Default for Database {
    fn default() -> Self { Self::new() }
}

impl Database {
    pub fn new() -> Self {
        Database {
            schema:  Schema::new(),
            storage: MemStorage::new(),
            pragmas: PragmaState::default(),
            indexes: BTreeMap::new(),
        }
    }

    /// Register a table in the schema (used by `CREATE TABLE` handling).
    pub fn add_table(&mut self, table: Table) {
        self.storage.import(table.name.clone(), MemTable::new());
        self.schema.add_table(table);
    }

    /// Execute a SQL statement and return all result rows.
    ///
    /// DDL statements (CREATE TABLE) update the schema; DML/DQL statements
    /// return zero or more rows of `Register` values.
    pub fn execute(&mut self, sql: &str) -> Result<Vec<Vec<Register>>, DbError> {
        let stmts = parse_sql(sql)
            .map_err(|e| DbError::Parse(format!("{e:?}")))?;

        let mut all_rows = Vec::new();
        for stmt in &stmts {
            // Handle CREATE TABLE by updating the schema directly.
            if let Some(table) = Self::stmt_as_create_table(stmt) {
                self.storage.import(table.name.clone(), MemTable::new());
                self.schema.add_table(table);
                continue;
            }

            let tables_snapshot: BTreeMap<String, MemTable> = self
                .storage
                .snapshot_tables();

            let output = compile(
                stmt,
                &self.schema,
                &self.pragmas,
                tables_snapshot,
                self.indexes.clone(),
            )
            .map_err(|e| DbError::Compile(e.to_string()))?;

            // Import any compile-time materialised subquery tables.
            for (name, tbl) in output.materialized_tables {
                self.storage.import(name, tbl);
            }

            let mut vm = Vm::new(256, output.prog.n_vars, self.storage.clone());
            let prog = output.prog;

            loop {
                let rc = step(&prog, &mut vm)
                    .map_err(|e| DbError::Exec(format!("{e:?}")))?;
                if rc == SQLITE_ROW {
                    all_rows.push(vm.result_row.clone());
                } else if rc == SQLITE_DONE {
                    break;
                } else {
                    return Err(DbError::Exec(format!("step returned {rc}")));
                }
            }

            // Flush any mutations back from the VM's storage snapshot.
            self.storage.merge(vm.backend);
        }

        Ok(all_rows)
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn stmt_as_create_table(stmt: &Stmt) -> Option<Table> {
        if let Stmt::CreateTable(ct) = stmt {
            let columns = match &ct.body {
                CreateTableBody::Columns { columns, .. } => columns,
                CreateTableBody::As(_) => return None, // CREATE TABLE AS SELECT not handled
            };
            let cols = columns.iter().map(|c| {
                let declared_type = c.type_name.as_ref()
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| "TEXT".into());
                let not_null    = c.constraints.iter().any(|con| matches!(con, ColumnConstraint::NotNull { .. }));
                let primary_key = c.constraints.iter().any(|con| matches!(con, ColumnConstraint::PrimaryKey { .. }));
                let default_value = c.constraints.iter().find_map(|con| {
                    if let ColumnConstraint::Default(e) = con { Some(format!("{e:?}")) } else { None }
                });
                Column { name: c.name.clone(), declared_type, not_null, primary_key, default_value }
            }).collect();
            Some(Table { name: ct.name.to_ascii_lowercase(), columns: cols, without_rowid: ct.without_rowid })
        } else {
            None
        }
    }
}
