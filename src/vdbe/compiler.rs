//! SQL statement compiler: AST → VDBE bytecode.
//!
//! # Locking discipline
//!
//! `lock_global()` is called ONLY inside `compile()` (the public entry point),
//! never inside any `compile_*` method.  All global state needed by the compiler
//! is snapshotted before `compile_stmt()` is called and threaded through the
//! `Compiler` struct fields.

use std::collections::BTreeMap;

use crate::schema::{Schema, Table, Column};
use crate::sql::ast::*;
use crate::wip_db::{
IndexData, JournalMode, PragmaState,
};
use crate::vdbe::exec::MemTable;

use super::opcodes::{Opcode, P4, Instr, Program};
use super::error::CompileError;
use super::storage::MemStorage;

// ── Public output type ────────────────────────────────────────────────────────

/// Everything produced by a single `compile()` call.
pub struct CompileOutput {
    pub prog: Program,
    /// CTE / subquery table schemas that must be registered in global state
    /// before the program is executed.
    pub cte_schemas: Vec<Table>,
    /// Subquery rows that were materialised at compile time (table_name → rows).
    /// The caller must import these into the Vm's MemStorage via `backend.import()`
    /// before calling `step()`.
    pub materialized_tables: BTreeMap<String, MemTable>,
}

// ── Scan context ──────────────────────────────────────────────────────────────

#[derive(Clone)]
struct ScanSource<'a> {
    cursor: i32,
    table: &'a Table,
    alias: Option<String>,
}

struct ScanCtx<'a> {
    sources: Vec<ScanSource<'a>>,
}

// ── Compiler ──────────────────────────────────────────────────────────────────

struct Compiler<'s> {
    prog: Program,
    next_reg: i32,
    next_cursor: i32,
    next_subq: i32,
    schema: &'s Schema,
    named_params: std::collections::HashMap<String, i32>,

    // Snapshots passed in from compile() — no lock needed after construction.
    tables: BTreeMap<String, MemTable>,
    pragmas: &'s PragmaState,
    indexes: BTreeMap<String, IndexData>,

    // Side-effects collected during compilation, returned in CompileOutput.
    cte_schemas: Vec<Table>,
    materialized_tables: BTreeMap<String, MemTable>,
}

impl<'s> Compiler<'s> {
    fn new(
        schema: &'s Schema,
        pragmas: &'s PragmaState,
        tables: BTreeMap<String, MemTable>,
        indexes: BTreeMap<String, IndexData>,
    ) -> Self {
        Compiler {
            prog: Program::default(),
            next_reg: 0,
            next_cursor: 0,
            next_subq: 0,
            schema,
            named_params: std::collections::HashMap::new(),
            tables,
            pragmas,
            indexes,
            cte_schemas: Vec::new(),
            materialized_tables: BTreeMap::new(),
        }
    }

    /// Build a MemStorage seeded with the compiler's current table snapshot.
    fn snapshot_storage(&self) -> MemStorage {
        let mut s = MemStorage::new();
        for (name, tbl) in &self.tables {
            s.import(name.clone(), tbl.clone());
        }
        s
    }

    fn alloc_reg(&mut self) -> i32 {
        let r = self.next_reg;
        self.next_reg += 1;
        r
    }
    fn alloc_regs(&mut self, n: i32) -> i32 {
        let base = self.next_reg;
        self.next_reg += n;
        base
    }
    fn alloc_cursor(&mut self) -> i32 {
        let c = self.next_cursor;
        self.next_cursor += 1;
        c
    }

    // ── CTE helper ────────────────────────────────────────────────────────────

    /// Register CTE column schemas so the rest of compilation can resolve them.
    /// Does NOT touch global state — schemas are collected in `self.cte_schemas`
    /// and returned to the caller via `CompileOutput`.
    fn register_cte_schemas(&mut self, with_clause: &WithClause) {
        for cte in &with_clause.ctes {
            let table_name = cte.name.to_ascii_lowercase();
            let columns: Vec<Column> = cte.columns.iter().map(|name| Column {
                name: name.clone(),
                declared_type: "TEXT".to_string(),
                not_null: false,
                primary_key: false,
                default_value: None,
            }).collect();
            let table = Table {
                name: table_name.clone(),
                columns,
                without_rowid: false,
            };
            // Make visible to the rest of this compile pass via schema snapshot.
            // (The caller will register them in global state after compile returns.)
            self.cte_schemas.push(table);
        }
    }

    // ── Subquery materialisation ──────────────────────────────────────────────

    /// Materialise a subquery into a temporary MemTable.
    /// Results are stored in `self.materialized_tables` and `self.tables` so
    /// subsequent references within this compile pass resolve correctly.
    /// No global state is touched.
    fn materialize_subquery(
        &mut self,
        select: &SelectStmt,
        _alias: &Option<String>,
    ) -> Result<String, CompileError> {
        use crate::vdbe::exec::{Vm, step};

        let subquery_prog = self.compile_subquery(select)?;
        let mut vm = Vm::new(256, subquery_prog.n_vars, self.snapshot_storage());

        let mut rows: Vec<Vec<crate::vdbe::exec::Register>> = Vec::new();
        loop {
            match step(&subquery_prog, &mut vm) {
                Ok(100) => rows.push(vm.result_row.clone()),
                Ok(_) => break,
                Err(e) => return Err(CompileError::new(format!("subquery execution failed: {e}"))),
            }
        }

        let table_name = format!("__subq_{}", self.next_subq);
        self.next_subq += 1;

        let n_cols = rows.first().map(|r| r.len()).unwrap_or(0);
        let columns: Vec<Column> = select.core.columns.iter().enumerate()
            .take(n_cols)
            .map(|(i, col)| {
                let col_name = match col {
                    ResultColumn::Expr { alias: Some(name), .. } => name.clone(),
                    ResultColumn::Expr { expr, alias: None } => match expr {
                        Expr::Column { name, .. } => name.clone(),
                        _ => format!("col{i}"),
                    },
                    _ => format!("col{i}"),
                };
                Column { name: col_name, declared_type: "TEXT".into(),
                         not_null: false, primary_key: false, default_value: None }
            })
            .collect();

        let table = Table { name: table_name.clone(), columns, without_rowid: false };
        self.cte_schemas.push(table);

        let mut mem = MemTable::new();
        for (rowid, row) in rows.into_iter().enumerate() {
            mem.rows.insert((rowid + 1) as i64, row);
        }
        mem.next_rowid = mem.rows.len() as i64 + 1;
        // Make it visible to subsequent materialisations in this compile pass.
        self.tables.insert(table_name.clone(), mem.clone());
        self.materialized_tables.insert(table_name.clone(), mem);

        Ok(table_name)
    }

    /// Compile a SELECT sub-statement using the same snapshots as the parent
    /// compiler.  Used for subquery materialisation and sub-programs.
    fn compile_subquery(&self, select: &SelectStmt) -> Result<Program, CompileError> {
        let mut sub = Compiler::new(self.schema, self.pragmas, self.tables.clone(), self.indexes.clone());
        sub.next_subq = self.next_subq;
        sub.compile_stmt(&Stmt::Select(select.clone()))?;
        Ok(sub.prog)
    }

    // ── Expression compiler ───────────────────────────────────────────────────

    fn compile_expr(
        &mut self,
        expr: &Expr,
        ctx: Option<&ScanCtx<'_>>,
    ) -> Result<i32, CompileError> {
        match expr {
            Expr::Literal(lit) => self.compile_literal(lit),

            Expr::BindParam(bp) => {
                let dst = self.alloc_reg();
                let (var_num, p4) = match bp {
                    BindParam::Positional => {
                        self.prog.n_vars += 1;
                        (self.prog.n_vars as i32, P4::None)
                    }
                    BindParam::PositionalN(n) => {
                        if *n as u32 > self.prog.n_vars {
                            self.prog.n_vars = *n as u32;
                        }
                        (*n as i32, P4::None)
                    }
                    BindParam::Named(name) => {
                        if let Some(&existing_num) = self.named_params.get(name) {
                            (existing_num, P4::Text(name.clone()))
                        } else {
                            self.prog.n_vars += 1;
                            let var_num = self.prog.n_vars as i32;
                            self.named_params.insert(name.clone(), var_num);
                            (var_num, P4::Text(name.clone()))
                        }
                    }
                };
                self.prog.emit(Instr::new(Opcode::Variable, var_num, dst, 0).p4(p4));
                Ok(dst)
            }

            Expr::Column { table: tbl_name, name, .. } => {
                let ctx = ctx.ok_or_else(|| {
                    CompileError::new(format!("column '{name}' referenced outside a FROM clause"))
                })?;

                let mut matches = Vec::new();
                for src in &ctx.sources {
                    let table_matches = if let Some(t) = tbl_name {
                        src.table.name.eq_ignore_ascii_case(t)
                            || src.alias.as_deref()
                                .map(|a| a.eq_ignore_ascii_case(t))
                                .unwrap_or(false)
                    } else {
                        true
                    };
                    if table_matches {
                        if let Some(col_idx) = src.table.columns.iter()
                            .position(|c| c.name.eq_ignore_ascii_case(name))
                        {
                            matches.push((src.cursor, col_idx));
                        }
                    }
                }

                if matches.is_empty() {
                    return Err(CompileError::new(format!("no such column: {name}")));
                }
                if matches.len() > 1 {
                    return Err(CompileError::new(format!("ambiguous column name: {name}")));
                }

                let (cursor, col_idx) = matches[0];
                let dst = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Column, cursor, col_idx as i32, dst));
                Ok(dst)
            }

            Expr::UnaryOp { op, expr } => {
                let src = self.compile_expr(expr, ctx)?;
                let dst = self.alloc_reg();
                match op {
                    UnaryOp::Neg => {
                        let zero = self.alloc_reg();
                        self.prog.emit(Instr::new(Opcode::Integer, 0, zero, 0));
                        self.prog.emit(Instr::new(Opcode::Subtract, zero, src, dst));
                    }
                    UnaryOp::Pos => {
                        self.prog.emit(Instr::new(Opcode::Copy, src, dst, 0));
                    }
                    UnaryOp::Not => {
                        self.prog.emit(Instr::new(Opcode::Not, src, dst, 0));
                    }
                    UnaryOp::BitNot => {
                        self.prog.emit(Instr::new(Opcode::BitNot, src, dst, 0));
                    }
                }
                Ok(dst)
            }

            Expr::BinOp { op, left, right } => {
                use BinOp::*;
                let lreg = self.compile_expr(left, ctx)?;
                let rreg = self.compile_expr(right, ctx)?;
                let dst = self.alloc_reg();
                match op {
                    Eq | Ne | Lt | Le | Gt | Ge => {
                        self.prog.emit(Instr::new(Opcode::Null, 0, dst, 0));
                        let skip1 = self.prog.emit(Instr::new(Opcode::IsNull, lreg, 0, 0));
                        let skip2 = self.prog.emit(Instr::new(Opcode::IsNull, rreg, 0, 0));
                        self.prog.emit(Instr::new(Opcode::Integer, 0, dst, 0));
                        let pos = match op {
                            Eq => Opcode::Eq, Ne => Opcode::Ne,
                            Lt => Opcode::Lt, Le => Opcode::Le,
                            Gt => Opcode::Gt, Ge => Opcode::Ge,
                            _ => unreachable!(),
                        };
                        let over = self.prog.emit(Instr::new(pos, lreg, 0, rreg));
                        let skip3 = self.prog.emit(Instr::new(Opcode::Goto, 0, 0, 0));
                        let over_addr = self.prog.next_addr();
                        self.prog.emit(Instr::new(Opcode::Integer, 1, dst, 0));
                        let skip_addr = self.prog.next_addr();
                        self.prog.patch_p2(skip1, skip_addr);
                        self.prog.patch_p2(skip2, skip_addr);
                        self.prog.patch_p2(over, over_addr);
                        self.prog.patch_p2(skip3, skip_addr);
                    }
                    And => { self.prog.emit(Instr::new(Opcode::And, lreg, rreg, dst)); }
                    Or  => { self.prog.emit(Instr::new(Opcode::Or,  lreg, rreg, dst)); }
                    Add => { self.prog.emit(Instr::new(Opcode::Add, lreg, rreg, dst)); }
                    Sub => { self.prog.emit(Instr::new(Opcode::Subtract,  lreg, rreg, dst)); }
                    Mul => { self.prog.emit(Instr::new(Opcode::Multiply,  lreg, rreg, dst)); }
                    Div => { self.prog.emit(Instr::new(Opcode::Divide,    lreg, rreg, dst)); }
                    Rem => { self.prog.emit(Instr::new(Opcode::Remainder, lreg, rreg, dst)); }
                    BitAnd  => { self.prog.emit(Instr::new(Opcode::BitAnd,    lreg, rreg, dst)); }
                    BitOr   => { self.prog.emit(Instr::new(Opcode::BitOr,     lreg, rreg, dst)); }
                    LShift  => { self.prog.emit(Instr::new(Opcode::ShiftLeft,  lreg, rreg, dst)); }
                    RShift  => { self.prog.emit(Instr::new(Opcode::ShiftRight, lreg, rreg, dst)); }
                    Concat  => { self.prog.emit(Instr::new(Opcode::Concat,     lreg, rreg, dst)); }
                }
                Ok(dst)
            }

            Expr::IsNull { not, expr } => {
                let src = self.compile_expr(expr, ctx)?;
                let dst = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Integer, 0, dst, 0));
                let skip_op = if *not { Opcode::IsNull } else { Opcode::NotNull };
                let skip = self.prog.emit(Instr::new(skip_op, src, 0, 0));
                self.prog.emit(Instr::new(Opcode::Integer, 1, dst, 0));
                let next = self.prog.next_addr();
                self.prog.patch_p2(skip, next);
                Ok(dst)
            }

            Expr::Is { not, left, right } => {
                let lreg = self.compile_expr(left, ctx)?;
                let rreg = self.compile_expr(right, ctx)?;
                let dst = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Integer, 0, dst, 0));
                let cmp_op = if *not { Opcode::Ne } else { Opcode::Eq };
                let skip = self.prog.emit(Instr::new(cmp_op, lreg, 0, rreg).p5(0x80));
                self.prog.emit(Instr::new(Opcode::Integer, 1, dst, 0));
                let next = self.prog.next_addr();
                self.prog.patch_p2(skip, next);
                Ok(dst)
            }

            Expr::Between { not, expr, lo, hi } => {
                let val    = self.compile_expr(expr, ctx)?;
                let lo_reg = self.compile_expr(lo,   ctx)?;
                let hi_reg = self.compile_expr(hi,   ctx)?;
                let dst = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Integer, 0, dst, 0));
                let j1 = self.prog.emit(Instr::new(Opcode::Lt, val, 0, lo_reg));
                let j2 = self.prog.emit(Instr::new(Opcode::Gt, val, 0, hi_reg));
                self.prog.emit(Instr::new(Opcode::Integer, 1, dst, 0));
                let done = self.prog.next_addr();
                self.prog.patch_p2(j1, done);
                self.prog.patch_p2(j2, done);
                if *not { self.prog.emit(Instr::new(Opcode::Not, dst, dst, 0)); }
                Ok(dst)
            }

            Expr::Like { not, expr, pattern, .. } => {
                let val_reg = self.compile_expr(expr,    ctx)?;
                let pat_reg = self.compile_expr(pattern, ctx)?;
                let dst = self.alloc_reg();
                self.prog.emit(
                    Instr::new(Opcode::Function, 2, val_reg, dst)
                        .p4(P4::FuncName("like".into())),
                );
                let _ = pat_reg;
                if *not { self.prog.emit(Instr::new(Opcode::Not, dst, dst, 0)); }
                Ok(dst)
            }

            Expr::In { not, expr, set } => {
                match set {
                    InSet::List(values) if !values.is_empty() => {
                        let val = self.compile_expr(expr, ctx)?;
                        let dst = self.alloc_reg();
                        self.prog.emit(Instr::new(Opcode::Integer, 0, dst, 0));
                        let mut found_jumps = Vec::new();
                        for item in values {
                            let item_reg = self.compile_expr(item, ctx)?;
                            let j = self.prog.emit(Instr::new(Opcode::Eq, val, 0, item_reg));
                            found_jumps.push(j);
                        }
                        let skip = self.prog.emit(Instr::new(Opcode::Goto, 0, 0, 0));
                        let found = self.prog.next_addr();
                        self.prog.emit(Instr::new(Opcode::Integer, 1, dst, 0));
                        let done = self.prog.next_addr();
                        self.prog.patch_p2(skip, done);
                        for j in found_jumps { self.prog.patch_p2(j, found); }
                        if *not { self.prog.emit(Instr::new(Opcode::Not, dst, dst, 0)); }
                        Ok(dst)
                    }
                    InSet::List(_) => {
                        let dst = self.alloc_reg();
                        let _ = self.compile_expr(expr, ctx)?;
                        self.prog.emit(Instr::new(Opcode::Integer, if *not { 1 } else { 0 }, dst, 0));
                        Ok(dst)
                    }
                    InSet::Select(select_stmt) => {
                        use crate::vdbe::exec::{Vm, step, Register};
                        let val = self.compile_expr(expr, ctx)?;

                        // Materialise subquery using snapshot tables — no lock needed.
                        let subquery_prog = self.compile_subquery(select_stmt)?;
                        let mut vm = Vm::new(256, subquery_prog.n_vars, self.snapshot_storage());

                        let mut result_values: Vec<Register> = Vec::new();
                        loop {
                            match step(&subquery_prog, &mut vm) {
                                Ok(100) => {
                                    if !vm.result_row.is_empty() {
                                        result_values.push(vm.result_row[0].clone());
                                    }
                                }
                                Ok(_) => break,
                                Err(_) => { result_values.clear(); break; }
                            }
                        }

                        if result_values.is_empty() {
                            let dst = self.alloc_reg();
                            self.prog.emit(Instr::new(Opcode::Integer, if *not { 1 } else { 0 }, dst, 0));
                            Ok(dst)
                        } else {
                            let dst = self.alloc_reg();
                            self.prog.emit(Instr::new(Opcode::Integer, 0, dst, 0));
                            let mut found_jumps = Vec::new();
                            for result_val in result_values {
                                let item_reg = self.alloc_reg();
                                match result_val {
                                    Register::Null       => { self.prog.emit(Instr::new(Opcode::Null,    0,                  item_reg, 0)); }
                                    Register::Integer(i) => { self.prog.emit(Instr::new(Opcode::Int64,   0,                  item_reg, 0).p4(P4::Int(i))); }
                                    Register::Real(f)    => { self.prog.emit(Instr::new(Opcode::Real,    0,                  item_reg, 0).p4(P4::Real(f))); }
                                    Register::Text(s)    => { self.prog.emit(Instr::new(Opcode::String8, 0,                  item_reg, 0).p4(P4::Text(s))); }
                                    Register::Blob(b)    => { self.prog.emit(Instr::new(Opcode::Blob,    b.len() as i32,     item_reg, 0).p4(P4::Blob(b))); }
                                }
                                let j = self.prog.emit(Instr::new(Opcode::Eq, val, 0, item_reg));
                                found_jumps.push(j);
                            }
                            let skip = self.prog.emit(Instr::new(Opcode::Goto, 0, 0, 0));
                            let found = self.prog.next_addr();
                            self.prog.emit(Instr::new(Opcode::Integer, 1, dst, 0));
                            let done = self.prog.next_addr();
                            self.prog.patch_p2(skip, done);
                            for j in found_jumps { self.prog.patch_p2(j, found); }
                            if *not { self.prog.emit(Instr::new(Opcode::Not, dst, dst, 0)); }
                            Ok(dst)
                        }
                    }
                    InSet::TableName(_) => {
                        let _val = self.compile_expr(expr, ctx)?;
                        let dst = self.alloc_reg();
                        self.prog.emit(Instr::new(Opcode::Integer, if *not { 1 } else { 0 }, dst, 0));
                        Ok(dst)
                    }
                }
            }

            Expr::Case { base, branches, else_ } => {
                let dst = self.alloc_reg();
                let mut end_jumps = Vec::new();
                if let Some(base_expr) = base {
                    let base_reg = self.compile_expr(base_expr, ctx)?;
                    for (cond, then) in branches {
                        let cond_reg = self.compile_expr(cond, ctx)?;
                        let match_jump = self.prog.emit(Instr::new(Opcode::Eq, base_reg, 0, cond_reg));
                        let skip = self.prog.emit(Instr::new(Opcode::Goto, 0, 0, 0));
                        let then_addr = self.prog.next_addr();
                        self.prog.patch_p2(match_jump, then_addr);
                        let val = self.compile_expr(then, ctx)?;
                        self.prog.emit(Instr::new(Opcode::Copy, val, dst, 0));
                        let end = self.prog.emit(Instr::new(Opcode::Goto, 0, 0, 0));
                        end_jumps.push(end);
                        let next = self.prog.next_addr();
                        self.prog.patch_p2(skip, next);
                    }
                } else {
                    for (cond, then) in branches {
                        let creg = self.compile_expr(cond, ctx)?;
                        let skip = self.prog.emit(Instr::new(Opcode::IfNot, creg, 0, 0));
                        let val = self.compile_expr(then, ctx)?;
                        self.prog.emit(Instr::new(Opcode::Copy, val, dst, 0));
                        let end = self.prog.emit(Instr::new(Opcode::Goto, 0, 0, 0));
                        end_jumps.push(end);
                        let next = self.prog.next_addr();
                        self.prog.patch_p2(skip, next);
                    }
                }
                if let Some(e) = else_ {
                    let val = self.compile_expr(e, ctx)?;
                    self.prog.emit(Instr::new(Opcode::Copy, val, dst, 0));
                } else {
                    self.prog.emit(Instr::new(Opcode::Null, 0, dst, 0));
                }
                let done = self.prog.next_addr();
                for j in end_jumps { self.prog.patch_p2(j, done); }
                Ok(dst)
            }

            Expr::FunctionCall { name, distinct, args, filter: _ } => {
                let lower_name = name.to_ascii_lowercase();
                let is_aggregate = matches!(lower_name.as_str(),
                    "count" | "sum" | "avg" | "min" | "max" | "group_concat" | "string_agg");
                if *distinct && !is_aggregate {
                    return Err(CompileError::new(
                        "DISTINCT can only be used with aggregate functions",
                    ));
                }
                let func_name = if *distinct && is_aggregate {
                    format!("{}#d", lower_name)
                } else {
                    lower_name
                };
                let base = self.next_reg;
                let n = args.len() as i32;
                for arg in args { self.compile_expr(arg, ctx)?; }
                let dst = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Function, n, base, dst).p4(P4::FuncName(func_name)));
                Ok(dst)
            }

            Expr::Cast { expr, .. } => {
                let src = self.compile_expr(expr, ctx)?;
                let dst = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Copy, src, dst, 0));
                Ok(dst)
            }

            Expr::Collate { expr, .. } => self.compile_expr(expr, ctx),

            Expr::Exists(select_stmt) => {
                use crate::vdbe::exec::{Vm, step};
                let subquery_prog = self.compile_subquery(select_stmt)?;
                let mut vm = Vm::new(256, subquery_prog.n_vars, self.snapshot_storage());
                let mut exists = 0i64;
                loop {
                    match step(&subquery_prog, &mut vm) {
                        Ok(100) => { exists = 1; break; }
                        Ok(_)   => break,
                        Err(e)  => {
                            eprintln!("Warning: EXISTS subquery evaluation failed: {e}, treating as FALSE");
                            break;
                        }
                    }
                }
                let dst = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Integer, exists as i32, dst, 0));
                Ok(dst)
            }

            Expr::Subquery(select_stmt) => {
                use crate::vdbe::exec::{Vm, step, Register};
                let subquery_prog = self.compile_subquery(select_stmt)?;
                let mut vm = Vm::new(256, subquery_prog.n_vars, self.snapshot_storage());
                let mut result_value = Register::Null;
                loop {
                    match step(&subquery_prog, &mut vm) {
                        Ok(100) => {
                            if !vm.result_row.is_empty() {
                                result_value = vm.result_row[0].clone();
                            }
                            break;
                        }
                        Ok(_)  => break,
                        Err(e) => {
                            eprintln!("Warning: scalar subquery evaluation failed: {e}, treating as NULL");
                            break;
                        }
                    }
                }
                let dst = self.alloc_reg();
                match &result_value {
                    Register::Null       => { self.prog.emit(Instr::new(Opcode::Null,    0,              dst, 0)); }
                    Register::Integer(n) => { self.prog.emit(Instr::new(Opcode::Integer, *n as i32,      dst, 0)); }
                    Register::Real(r)    => { self.prog.emit(Instr::new(Opcode::Integer, *r as i32,      dst, 0)); }
                    Register::Text(s)    => { self.prog.emit(Instr::new(Opcode::String8, 0,              dst, 0).p4(P4::Text(s.clone()))); }
                    Register::Blob(b)    => { self.prog.emit(Instr::new(Opcode::Blob,    b.len() as i32, dst, 0).p4(P4::Blob(b.clone()))); }
                }
                Ok(dst)
            }

            Expr::Raise { .. } => {
                let dst = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Null, 0, dst, 0));
                Ok(dst)
            }
        }
    }

    fn compile_literal(&mut self, lit: &Literal) -> Result<i32, CompileError> {
        let dst = self.alloc_reg();
        match lit {
            Literal::Null             => { self.prog.emit(Instr::new(Opcode::Null,    0,          dst, 0)); }
            Literal::True             => { self.prog.emit(Instr::new(Opcode::Integer, 1,          dst, 0)); }
            Literal::False            => { self.prog.emit(Instr::new(Opcode::Integer, 0,          dst, 0)); }
            Literal::Real(v)          => { self.prog.emit(Instr::new(Opcode::Real,    0,          dst, 0).p4(P4::Real(*v))); }
            Literal::Text(s)          => { self.prog.emit(Instr::new(Opcode::String8, 0,          dst, 0).p4(P4::Text(s.clone()))); }
            Literal::Blob(b)          => { self.prog.emit(Instr::new(Opcode::Blob,    b.len() as i32, dst, 0).p4(P4::Blob(b.clone()))); }
            Literal::CurrentDate      => { self.prog.emit(Instr::new(Opcode::Function, 0, 0, dst).p4(P4::FuncName("current_date".into()))); }
            Literal::CurrentTime      => { self.prog.emit(Instr::new(Opcode::Function, 0, 0, dst).p4(P4::FuncName("current_time".into()))); }
            Literal::CurrentTimestamp => { self.prog.emit(Instr::new(Opcode::Function, 0, 0, dst).p4(P4::FuncName("current_timestamp".into()))); }
            Literal::Integer(v) => {
                if let Ok(v32) = i32::try_from(*v) {
                    self.prog.emit(Instr::new(Opcode::Integer, v32, dst, 0));
                } else {
                    self.prog.emit(Instr::new(Opcode::Int64, 0, dst, 0).p4(P4::Int(*v)));
                }
            }
        }
        Ok(dst)
    }

    // ── Statement compilers ───────────────────────────────────────────────────

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Select(s)   => self.compile_select(s),
            Stmt::Insert(s)   => self.compile_insert(s),
            Stmt::Update(s)   => self.compile_update(s),
            Stmt::Delete(s)   => self.compile_delete(s),
            Stmt::Pragma(s)   => self.compile_pragma(s),
            Stmt::Explain(inner) => self.compile_stmt(inner),
            Stmt::Begin(_) | Stmt::Commit | Stmt::Rollback => {
                self.prog.emit(Instr::new(Opcode::Transaction, 0, 0, 0));
                self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
                Ok(())
            }
            _ => {
                self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
                Ok(())
            }
        }
    }

    fn compile_select(&mut self, stmt: &SelectStmt) -> Result<(), CompileError> {
        // Collect CTE schemas — no global write, just push to self.cte_schemas.
        if let Some(with_clause) = &stmt.with {
            self.register_cte_schemas(with_clause);
        }

        let core = &stmt.core;
        if !stmt.compounds.is_empty() {
            return self.compile_union(stmt);
        }

        let init_addr = self.prog.emit(Instr::new(Opcode::Init, 0, 0, 0));
        match &core.from {
            None       => self.compile_select_no_from(stmt, core, init_addr),
            Some(from) => self.compile_select_with_from(stmt, core, from, init_addr),
        }
    }

    fn compile_union(&mut self, stmt: &SelectStmt) -> Result<(), CompileError> {
        let init_addr = self.prog.emit(Instr::new(Opcode::Init, 0, 0, 0));
        let body = self.prog.next_addr();
        self.prog.patch_p2(init_addr, body);

        let left = SelectStmt {
            with: None, core: stmt.core.clone(),
            compounds: vec![], order_by: vec![], limit: None,
        };
        let left_prog = self.compile_subquery(&left)?;
        self.prog.emit(Instr::new(Opcode::Subprogram, 0, 0, 0).p4(P4::Program(Box::new(left_prog))));

        for compound in &stmt.compounds {
            let op = match compound.0 {
                crate::sql::ast::CompoundOp::Union     => 0,
                crate::sql::ast::CompoundOp::UnionAll  => 1,
                crate::sql::ast::CompoundOp::Intersect => 2,
                crate::sql::ast::CompoundOp::Except    => 3,
            };
            let branch = SelectStmt {
                with: None, core: compound.1.clone(),
                compounds: vec![], order_by: vec![], limit: None,
            };
            let branch_prog = self.compile_subquery(&branch)?;
            self.prog.emit(Instr::new(Opcode::Subprogram, op, 0, 0).p4(P4::Program(Box::new(branch_prog))));
        }

        self.prog.emit(Instr::new(Opcode::Union, 0, 0, 0));
        Ok(())
    }

    fn compile_select_no_from(
        &mut self,
        _stmt: &SelectStmt,
        core: &SelectCore,
        init_addr: usize,
    ) -> Result<(), CompileError> {
        let body = self.prog.next_addr();
        self.prog.patch_p2(init_addr, body);

        let mut result_regs: Vec<i32> = Vec::new();
        let mut col_names: Vec<String> = Vec::new();
        for col in &core.columns {
            match col {
                ResultColumn::Star | ResultColumn::TableStar(_) => {
                    return Err(CompileError::Error("no table specified".to_string()));
                }
                ResultColumn::Expr { expr, alias } => {
                    let col_name = if let Some(a) = alias { a.clone() } else {
                        match expr {
                            Expr::Column { name, .. }       => name.clone(),
                            Expr::FunctionCall { name, .. } => name.clone(),
                            _ => String::new(),
                        }
                    };
                    col_names.push(col_name);
                    result_regs.push(self.compile_expr(expr, None)?);
                }
            }
        }
        self.prog.column_names = col_names;

        let base  = self.next_reg;
        let count = result_regs.len() as i32;
        for src in result_regs {
            let dst = self.alloc_reg();
            if src != dst { self.prog.emit(Instr::new(Opcode::Copy, src, dst, 0)); }
        }

        let emit_result = |prog: &mut Program, distinct: bool, base: i32, count: i32| {
            if distinct {
                let ds = prog.emit(Instr::new(Opcode::DistinctCheck, base, 0, count));
                prog.emit(Instr::new(Opcode::ResultRow, base, count, 0));
                let after = prog.next_addr();
                prog.patch_p2(ds, after as i32);
            } else {
                prog.emit(Instr::new(Opcode::ResultRow, base, count, 0));
            }
        };

        if let Some(w) = &core.where_ {
            let cond = self.compile_expr(w, None)?;
            let skip = self.prog.emit(Instr::new(Opcode::IfNot, cond, 0, 0));
            emit_result(&mut self.prog, core.distinct, base, count);
            let done = self.prog.next_addr();
            self.prog.patch_p2(skip, done);
        } else {
            emit_result(&mut self.prog, core.distinct, base, count);
        }

        self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
        Ok(())
    }

    fn compile_select_with_from(
        &mut self,
        stmt: &SelectStmt,
        core: &SelectCore,
        from: &FromClause,
        init_addr: usize,
    ) -> Result<(), CompileError> {
        // ── Pre-process subqueries in FROM/JOIN ──────────────────────────────
        let subq_start_idx = self.next_subq;
        for item in &from.items {
            if let TableOrSubquery::Subquery { select, alias } = item {
                self.materialize_subquery(select, alias)?;
            }
        }
        for join in &from.joins {
            if let TableOrSubquery::Subquery { select, alias } = &join.table {
                self.materialize_subquery(select, alias)?;
            }
        }

        // ── Collect table names ──────────────────────────────────────────────
        let mut all_table_names:   Vec<String>       = Vec::new();
        let mut all_table_aliases: Vec<Option<String>> = Vec::new();

        let mut from_subq_counter = 0;
        for item in &from.items {
            let (tname, alias) = match item {
                TableOrSubquery::Table(t) => (t.name.to_ascii_lowercase(), t.alias.clone()),
                TableOrSubquery::Subquery { alias, .. } => {
                    let idx = subq_start_idx + from_subq_counter;
                    from_subq_counter += 1;
                    (format!("__subq_{idx}"), alias.clone())
                }
            };
            all_table_names.push(tname);
            all_table_aliases.push(alias);
        }

        let mut join_subq_counter = 0;
        for join in &from.joins {
            let (tname, table_alias) = match &join.table {
                TableOrSubquery::Table(t) => (t.name.to_ascii_lowercase(), t.alias.clone()),
                TableOrSubquery::Subquery { alias, .. } => {
                    let idx = subq_start_idx + from_subq_counter + join_subq_counter;
                    join_subq_counter += 1;
                    (format!("__subq_{idx}"), alias.clone())
                }
            };
            all_table_names.push(tname);
            all_table_aliases.push(table_alias);
        }

        // ── Resolve tables from schema snapshot — no lock needed ─────────────
        let mut all_tables: Vec<Table> = Vec::new();
        for tname in &all_table_names {
            let table = self.schema.get_table(tname).cloned()
                .or_else(|| {
                    // Check in materialized subquery schemas collected this pass.
                    self.cte_schemas.iter().find(|t| t.name.eq_ignore_ascii_case(tname)).cloned()
                })
                .ok_or_else(|| CompileError::new(format!("no such table: {tname}")))?;
            all_tables.push(table);
        }

        // ── Build sources ────────────────────────────────────────────────────
        let mut sources: Vec<ScanSource> = Vec::new();
        for (i, _) in all_table_names.iter().enumerate() {
            let cursor = self.alloc_cursor();
            sources.push(ScanSource {
                cursor,
                table: &all_tables[i],
                alias: all_table_aliases[i].clone(),
            });
        }

        // ── JOIN conditions ──────────────────────────────────────────────────
        let mut join_conditions: Vec<Option<Expr>> = Vec::new();
        let join_start_idx = all_table_names.len() - from.joins.len();
        for (i, join) in from.joins.iter().enumerate() {
            let actual_idx  = join_start_idx + i;
            let join_table  = &all_tables[actual_idx];
            let cond = match &join.constraint {
                Some(JoinConstraint::On(expr)) => Some(expr.clone()),
                Some(JoinConstraint::Using(cols)) => {
                    let mut combined: Option<Expr> = None;
                    for col in cols {
                        let eq = Expr::BinOp {
                            op: BinOp::Eq,
                            left:  Box::new(Expr::Column { schema: None, table: None, name: col.clone() }),
                            right: Box::new(Expr::Column { schema: None, table: None, name: col.clone() }),
                        };
                        combined = Some(match combined {
                            None       => eq,
                            Some(prev) => Expr::BinOp { op: BinOp::And, left: Box::new(prev), right: Box::new(eq) },
                        });
                    }
                    combined
                }
                None => {
                    if join.join_type == JoinType::Natural {
                        let mut combined: Option<Expr> = None;
                        let joined_alias = all_table_aliases[actual_idx].clone()
                            .unwrap_or_else(|| join_table.name.clone());
                        for col_name in &join_table.columns {
                            for prev_src in &sources[..actual_idx] {
                                if prev_src.table.columns.iter().any(|c| c.name.eq_ignore_ascii_case(&col_name.name)) {
                                    let prev_alias = prev_src.alias.clone()
                                        .unwrap_or_else(|| prev_src.table.name.clone());
                                    let eq = Expr::BinOp {
                                        op: BinOp::Eq,
                                        left:  Box::new(Expr::Column { schema: None, table: Some(prev_alias),    name: col_name.name.clone() }),
                                        right: Box::new(Expr::Column { schema: None, table: Some(joined_alias.clone()), name: col_name.name.clone() }),
                                    };
                                    combined = Some(match combined {
                                        None       => eq,
                                        Some(prev) => Expr::BinOp { op: BinOp::And, left: Box::new(prev), right: Box::new(eq) },
                                    });
                                    break;
                                }
                            }
                        }
                        combined
                    } else {
                        None
                    }
                }
            };
            join_conditions.push(cond);
        }

        let body = self.prog.next_addr();
        self.prog.patch_p2(init_addr, body);
        self.prog.emit(Instr::new(Opcode::Transaction, 0, 0, 0));

        // ── Index scan opportunity — uses self.indexes snapshot, no lock ─────
        if from.joins.is_empty() && sources.len() == 1 && core.where_.is_some()
            && core.columns.iter().all(|c| matches!(c, ResultColumn::Star | ResultColumn::TableStar(_)))
        {
            if let Some(Expr::BinOp { op: BinOp::Eq, left, right }) = &core.where_ {
                let (col_expr, lit_expr) = if matches!(left.as_ref(), Expr::Column { .. }) {
                    (left.as_ref(), right.as_ref())
                } else if matches!(right.as_ref(), Expr::Column { .. }) {
                    (right.as_ref(), left.as_ref())
                } else {
                    (&Expr::Literal(Literal::Null), &Expr::Literal(Literal::Null))
                };

                if let (Expr::Column { name, .. }, Expr::Literal(lit)) = (col_expr, lit_expr) {
                    let src = &sources[0];
                    if let Some(col_idx) = src.table.columns.iter()
                        .position(|c| c.name.eq_ignore_ascii_case(name))
                    {
                        let found_index = self.indexes.iter()
                            .find(|(_, idx)| {
                                idx.table.eq_ignore_ascii_case(&src.table.name)
                                    && idx.columns.len() == 1
                                    && idx.columns[0] == col_idx
                            })
                            .map(|(k, _)| k.clone());

                        if let Some(idx_name) = found_index {
                            let ncols = src.table.columns.len() as i32;
                            self.prog.emit(Instr::new(Opcode::OpenRead, src.cursor, 0, ncols)
                                .p4(P4::Text(src.table.name.clone())));
                            let key_reg = self.alloc_reg();
                            match lit {
                                Literal::Integer(v) => { self.prog.emit(Instr::new(Opcode::Integer, *v as i32, key_reg, 0)); }
                                Literal::Real(v)    => { self.prog.emit(Instr::new(Opcode::Real,    0,         key_reg, 0).p4(P4::Real(*v))); }
                                Literal::Text(t)    => { self.prog.emit(Instr::new(Opcode::String8, 0,         key_reg, 0).p4(P4::Text(t.clone()))); }
                                Literal::Null       => { self.prog.emit(Instr::new(Opcode::Null,    0,         key_reg, 0)); }
                                _ => {}
                            }
                            let not_found = self.prog.emit(Instr::new(Opcode::SeekEq, src.cursor, 0, key_reg)
                                .p4(P4::Text(idx_name)));
                            let loop_start = self.prog.next_addr() as i32;
                            let mut col_regs: Vec<i32> = Vec::new();
                            for i in 0..src.table.columns.len() {
                                let dst = self.alloc_reg();
                                self.prog.emit(Instr::new(Opcode::Column, src.cursor, i as i32, dst));
                                col_regs.push(dst);
                            }
                            self.prog.column_names = src.table.columns.iter().map(|c| c.name.clone()).collect();
                            let result_base  = *col_regs.first().unwrap_or(&0);
                            let result_count = col_regs.len() as i32;
                            self.prog.emit(Instr::new(Opcode::ResultRow, result_base, result_count, 0));
                            self.prog.emit(Instr::new(Opcode::IdxNext, src.cursor, loop_start, 0));
                            let after_loop = self.prog.next_addr() as i32;
                            self.prog.patch_p2(not_found, after_loop);
                            self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
                            return Ok(());
                        }
                    }
                }
            }
        }

        // ── Open cursors / Rewind ────────────────────────────────────────────
        let mut loop_controls: Vec<(i32, usize, i32)> = Vec::new();
        for (i, src) in sources.iter().enumerate() {
            let ncols = src.table.columns.len() as i32;
            self.prog.emit(Instr::new(Opcode::OpenRead, src.cursor, 0, ncols)
                .p4(P4::Text(src.table.name.clone())));
            let rewind     = self.prog.emit(Instr::new(Opcode::Rewind, src.cursor, 0, 0));
            let loop_start = self.prog.next_addr() as i32;
            if i > 0 {
                if let Some(Some(_)) = join_conditions.get(i - 1) {
                    // compiled below in join-condition block
                }
            }
            loop_controls.push((src.cursor, rewind, loop_start));
        }

        let ctx = ScanCtx { sources };

        // ── JOIN ON conditions ────────────────────────────────────────────────
        let mut skip_to_next: Vec<usize> = Vec::new();
        for cond_expr in &join_conditions {
            if let Some(expr) = cond_expr {
                let cond = self.compile_expr(expr, Some(&ctx))?;
                let j = self.prog.emit(Instr::new(Opcode::IfNot, cond, 0, 0));
                skip_to_next.push(j);
            }
        }

        // ── WHERE ─────────────────────────────────────────────────────────────
        if let Some(w) = &core.where_ {
            let cond = self.compile_expr(w, Some(&ctx))?;
            let j = self.prog.emit(Instr::new(Opcode::IfNot, cond, 0, 0));
            skip_to_next.push(j);
        }

        let has_aggregates = Self::has_aggregate(&core.columns);

        // ── Result columns ────────────────────────────────────────────────────
        let mut result_regs: Vec<i32> = Vec::new();
        let mut col_names:   Vec<String> = Vec::new();

        for col in &core.columns {
            match col {
                ResultColumn::Star => {
                    for src in &ctx.sources {
                        for (col_idx, table_col) in src.table.columns.iter().enumerate() {
                            let dst = self.alloc_reg();
                            self.prog.emit(Instr::new(Opcode::Column, src.cursor, col_idx as i32, dst));
                            result_regs.push(dst);
                            col_names.push(table_col.name.clone());
                        }
                    }
                }
                ResultColumn::TableStar(t) => {
                    let src = ctx.sources.iter()
                        .find(|s| s.table.name.eq_ignore_ascii_case(t)
                            || s.alias.as_deref().map(|a| a.eq_ignore_ascii_case(t)).unwrap_or(false))
                        .ok_or_else(|| CompileError::new(format!("unknown table '{t}'")))?;
                    for (i, table_col) in src.table.columns.iter().enumerate() {
                        let dst = self.alloc_reg();
                        self.prog.emit(Instr::new(Opcode::Column, src.cursor, i as i32, dst));
                        result_regs.push(dst);
                        col_names.push(table_col.name.clone());
                    }
                }
                ResultColumn::Expr { expr, alias } => {
                    let col_name = if let Some(a) = alias { a.clone() } else {
                        match expr {
                            Expr::Column { name, .. }       => name.clone(),
                            Expr::FunctionCall { name, .. } => name.clone(),
                            _ => String::new(),
                        }
                    };
                    col_names.push(col_name);

                    if has_aggregates {
                        if let Some(_func_name) = Self::extract_aggregate_func(expr) {
                            match expr {
                                Expr::FunctionCall { args, filter, .. } => {
                                    let dst = self.alloc_reg();
                                    self.prog.emit(Instr::new(Opcode::Null, 0, dst, 0));
                                    let skip_jump = if let Some(filter_expr) = filter {
                                        let cond = self.compile_expr(filter_expr, Some(&ctx))?;
                                        Some(self.prog.emit(Instr::new(Opcode::IfNot, cond, 0, 0)))
                                    } else { None };
                                    if args.is_empty() {
                                        self.prog.emit(Instr::new(Opcode::Integer, 1, dst, 0));
                                    } else {
                                        let val = self.compile_expr(&args[0], Some(&ctx))?;
                                        if val != dst { self.prog.emit(Instr::new(Opcode::Copy, val, dst, 0)); }
                                    }
                                    if let Some(j) = skip_jump {
                                        let after = self.prog.next_addr();
                                        self.prog.patch_p2(j, after as i32);
                                    }
                                    result_regs.push(dst);
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            result_regs.push(self.compile_expr(expr, Some(&ctx))?);
                        }
                    } else {
                        result_regs.push(self.compile_expr(expr, Some(&ctx))?);
                    }
                }
            }
        }

        self.prog.column_names = col_names;

        let result_base = self.next_reg;
        let out_count   = result_regs.len() as i32;
        for src in result_regs {
            let dst = self.alloc_reg();
            if src != dst { self.prog.emit(Instr::new(Opcode::Copy, src, dst, 0)); }
        }

        // ── ORDER BY / GROUP BY / plain output ────────────────────────────────
        let has_group_by = !core.group_by.is_empty();

        if has_group_by || has_aggregates {
            let mut sort_key_regs: Vec<i32> = Vec::new();
            for group_expr in &core.group_by {
                sort_key_regs.push(self.compile_expr(group_expr, Some(&ctx))?);
            }
            let sort_key_base = self.next_reg;
            for src in sort_key_regs {
                let dst = self.alloc_reg();
                if src != dst { self.prog.emit(Instr::new(Opcode::Copy, src, dst, 0)); }
            }
            let n_keys = (self.next_reg - sort_key_base) as i32;
            self.prog.emit(Instr::new(Opcode::SortRow, result_base, out_count, sort_key_base)
                .p4(P4::Int(n_keys as i64)));
        } else if !stmt.order_by.is_empty() {
            let mut sort_key_regs: Vec<i32> = Vec::new();
            for order_expr in &stmt.order_by {
                sort_key_regs.push(self.compile_expr(&order_expr.expr, Some(&ctx))?);
            }
            let sort_key_base = self.next_reg;
            for src in sort_key_regs {
                let dst = self.alloc_reg();
                if src != dst { self.prog.emit(Instr::new(Opcode::Copy, src, dst, 0)); }
            }
            let n_keys = (self.next_reg - sort_key_base) as i32;
            if core.distinct {
                let ds = self.prog.emit(Instr::new(Opcode::DistinctCheck, result_base, 0, out_count));
                self.prog.emit(Instr::new(Opcode::SortRow, result_base, out_count, sort_key_base).p4(P4::Int(n_keys as i64)));
                let after = self.prog.next_addr();
                self.prog.patch_p2(ds, after as i32);
            } else {
                self.prog.emit(Instr::new(Opcode::SortRow, result_base, out_count, sort_key_base).p4(P4::Int(n_keys as i64)));
            }
        } else if core.distinct {
            let ds = self.prog.emit(Instr::new(Opcode::DistinctCheck, result_base, 0, out_count));
            self.prog.emit(Instr::new(Opcode::ResultRow, result_base, out_count, 0));
            let after = self.prog.next_addr();
            self.prog.patch_p2(ds, after as i32);
        } else {
            self.prog.emit(Instr::new(Opcode::ResultRow, result_base, out_count, 0));
        }

        // ── Next (innermost → outermost) ──────────────────────────────────────
        let innermost_next_addr = self.prog.next_addr();
        for (cursor, _rewind, loop_start) in loop_controls.iter().rev() {
            self.prog.emit(Instr::new(Opcode::Next, *cursor, *loop_start, 0));
        }
        for j in skip_to_next {
            self.prog.patch_p2(j, innermost_next_addr as i32);
        }
        let end_of_select = self.prog.next_addr();
        for (_cursor, rewind, _loop_start) in &loop_controls {
            self.prog.patch_p2(*rewind, end_of_select as i32);
        }

        // ── SortEmit ──────────────────────────────────────────────────────────
        if has_group_by || has_aggregates {
            let mut mode_string = String::from("G:");
            let n_group = core.group_by.len();
            for _ in 0..n_group { mode_string.push('A'); }
            mode_string.push(':');
            for (i, col) in core.columns.iter().enumerate() {
                if i > 0 { mode_string.push(','); }
                let func = match col {
                    ResultColumn::Expr { expr, .. } =>
                        Self::extract_aggregate_func(expr).unwrap_or_else(|| "_".to_string()),
                    _ => "_".to_string(),
                };
                mode_string.push_str(&func);
            }
            self.prog.emit(Instr::new(Opcode::SortEmit, -1, 0, core.group_by.len() as i32)
                .p4(P4::Text(mode_string)));
        } else if !stmt.order_by.is_empty() {
            let mut dir_string = String::new();
            for (i, o) in stmt.order_by.iter().enumerate() {
                if i > 0 { dir_string.push(':'); }
                dir_string.push(if o.asc { 'A' } else { 'D' });
                dir_string.push(':');
            }
            self.prog.emit(Instr::new(Opcode::SortEmit, -1, 0, stmt.order_by.len() as i32)
                .p4(P4::Text(dir_string)));
        }

        self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
        Ok(())
    }

    fn compile_insert(&mut self, stmt: &InsertStmt) -> Result<(), CompileError> {
        if let Some(with_clause) = &stmt.with {
            self.register_cte_schemas(with_clause);
        }

        let tname = stmt.table.to_ascii_lowercase();
        let table = self.schema.get_table(&tname)
            .ok_or_else(|| CompileError::new(format!("no such table: {tname}")))?;
        let ncols  = table.columns.len() as i32;
        let cursor = self.alloc_cursor();

        let init_addr = self.prog.emit(Instr::new(Opcode::Init, 0, 0, 0));
        let body = self.prog.next_addr();
        self.prog.patch_p2(init_addr, body);
        self.prog.emit(Instr::new(Opcode::Transaction, 0, 1, 0));
        self.prog.emit(Instr::new(Opcode::OpenWrite, cursor, 0, ncols).p4(P4::Text(tname.clone())));

        let has_returning = !stmt.returning.is_empty();

        match &stmt.source {
            InsertSource::DefaultValues => {
                let rowid_reg = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::NewRowid, cursor, rowid_reg, 0));
                let first_reg = self.next_reg;
                for _ in 0..ncols {
                    let r = self.alloc_reg();
                    self.prog.emit(Instr::new(Opcode::Null, 0, r, 0));
                }
                let rec = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::MakeRecord, first_reg, ncols, rec));
                self.prog.emit(Instr::new(Opcode::Insert, cursor, rec, rowid_reg));
                if has_returning {
                    self.emit_returning(&stmt.returning, cursor, table, first_reg, ncols)?;
                }
            }
            InsertSource::Values(rows) => {
                for row in rows {
                    let rowid_reg = self.alloc_reg();
                    self.prog.emit(Instr::new(Opcode::NewRowid, cursor, rowid_reg, 0));
                    let first_reg = self.next_reg;
                    if stmt.columns.is_empty() {
                        if row.len() != ncols as usize {
                            return Err(CompileError::new(format!(
                                "column count mismatch: expected {ncols}, got {}", row.len()
                            )));
                        }
                        for val in row { self.compile_expr(val, None)?; }
                    } else {
                        let slots = self.alloc_regs(ncols);
                        for i in 0..ncols { self.prog.emit(Instr::new(Opcode::Null, 0, slots + i, 0)); }
                        for (col_name, val) in stmt.columns.iter().zip(row.iter()) {
                            let col_idx = table.columns.iter()
                                .position(|c| c.name.eq_ignore_ascii_case(col_name))
                                .ok_or_else(|| CompileError::new(format!("unknown column '{col_name}'")))?;
                            let tmp = self.compile_expr(val, None)?;
                            self.prog.emit(Instr::new(Opcode::Copy, tmp, slots + col_idx as i32, 0));
                        }
                    }
                    let rec = self.alloc_reg();
                    self.prog.emit(Instr::new(Opcode::MakeRecord, first_reg, ncols, rec));
                    self.prog.emit(Instr::new(Opcode::Insert, cursor, rec, rowid_reg));
                    if has_returning {
                        self.emit_returning(&stmt.returning, cursor, table, first_reg, ncols)?;
                    }
                }
            }
            InsertSource::Select(select_stmt) => {
                let select = SelectStmt {
                    with: None,
                    core: select_stmt.core.clone(),
                    compounds: select_stmt.compounds.clone(),
                    order_by: select_stmt.order_by.clone(),
                    limit: select_stmt.limit.clone(),
                };
                let select_prog = self.compile_subquery(&select)?;
                self.prog.emit(Instr::new(Opcode::Subprogram, 4, cursor, ncols)
                    .p4(P4::Program(Box::new(select_prog))));
            }
        }

        self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
        Ok(())
    }

    /// Emit ResultRow for a RETURNING clause.  Shared by INSERT, UPDATE, DELETE.
    fn emit_returning(
        &mut self,
        returning: &[ResultColumn],
        cursor: i32,
        table: &Table,
        first_reg: i32,
        ncols: i32,
    ) -> Result<(), CompileError> {
        let ctx = ScanCtx {
            sources: vec![ScanSource { cursor, table, alias: None }],
        };
        let mut result_regs: Vec<i32> = Vec::new();
        for ret_col in returning {
            match ret_col {
                ResultColumn::Expr { expr, .. } => {
                    result_regs.push(self.compile_expr(expr, Some(&ctx))?);
                }
                ResultColumn::Star => {
                    for i in 0..ncols { result_regs.push(first_reg + i); }
                }
                ResultColumn::TableStar(tbl_name) => {
                    if tbl_name.eq_ignore_ascii_case(&table.name) {
                        for i in 0..ncols { result_regs.push(first_reg + i); }
                    } else {
                        return Err(CompileError::new(format!("unknown table '{tbl_name}' in RETURNING")));
                    }
                }
            }
        }
        if !result_regs.is_empty() {
            let base  = result_regs[0];
            let count = result_regs.len() as i32;
            self.prog.emit(Instr::new(Opcode::ResultRow, base, count, 0));
        }
        Ok(())
    }

    fn compile_update(&mut self, stmt: &UpdateStmt) -> Result<(), CompileError> {
        if let Some(with_clause) = &stmt.with {
            self.register_cte_schemas(with_clause);
        }

        let mut from_tables:  Vec<Table>       = Vec::new();
        let mut from_cursors: Vec<i32>          = Vec::new();
        let mut from_sources: Vec<ScanSource>   = Vec::new();

        if let Some(from) = &stmt.from {
            for item in &from.items {
                if let TableOrSubquery::Subquery { select, .. } = item {
                    self.materialize_subquery(select, &None)?;
                }
            }
            for item in &from.items {
                let tname = match item {
                    TableOrSubquery::Table(t) => t.name.to_ascii_lowercase(),
                    TableOrSubquery::Subquery { .. } => format!("__subq_{}", self.next_subq - 1),
                };
                let table = self.schema.get_table(&tname).cloned()
                    .or_else(|| self.cte_schemas.iter().find(|t| t.name.eq_ignore_ascii_case(&tname)).cloned())
                    .ok_or_else(|| CompileError::new(format!("no such table: {tname}")))?;
                from_tables.push(table);
            }
            for table in &from_tables {
                let cursor = self.alloc_cursor();
                from_cursors.push(cursor);
                from_sources.push(ScanSource { cursor, table, alias: None });
            }
        }

        let tname  = stmt.table.to_ascii_lowercase();
        let table  = self.schema.get_table(&tname)
            .ok_or_else(|| CompileError::new(format!("no such table: {tname}")))?;
        let ncols  = table.columns.len() as i32;
        let cursor = self.alloc_cursor();

        let init_addr = self.prog.emit(Instr::new(Opcode::Init, 0, 0, 0));
        let body = self.prog.next_addr();
        self.prog.patch_p2(init_addr, body);
        self.prog.emit(Instr::new(Opcode::Transaction, 0, 1, 0));
        self.prog.emit(Instr::new(Opcode::OpenWrite, cursor, 0, ncols).p4(P4::Text(tname)));

        for (i, from_cursor) in from_cursors.iter().enumerate() {
            let from_ncols = from_tables[i].columns.len() as i32;
            self.prog.emit(Instr::new(Opcode::OpenRead, *from_cursor, 0, from_ncols)
                .p4(P4::Text(from_tables[i].name.clone())));
        }

        let rewind = self.prog.emit(Instr::new(Opcode::Rewind, cursor, 0, 0));
        let main_loop_start = self.prog.next_addr() as i32;

        let from_inner_rewinds: Vec<usize> = from_cursors.iter()
            .map(|fc| self.prog.emit(Instr::new(Opcode::Rewind, *fc, 0, 0)))
            .collect();
        let from_loop_start = self.prog.next_addr() as i32;

        let mut all_sources = vec![ScanSource { cursor, table: &table, alias: stmt.alias.clone() }];
        all_sources.extend(from_sources.iter().cloned());
        let ctx = ScanCtx { sources: all_sources };

        let mut skip_to_next: Vec<usize> = Vec::new();
        if let Some(w) = &stmt.where_ {
            let cond = self.compile_expr(w, Some(&ctx))?;
            let j = self.prog.emit(Instr::new(Opcode::IfNot, cond, 0, 0));
            skip_to_next.push(j);
        }

        let rowid_reg = self.alloc_reg();
        self.prog.emit(Instr::new(Opcode::Rowid, cursor, rowid_reg, 0));
        let first_reg = self.next_reg;
        for i in 0..ncols {
            let r = self.alloc_reg();
            self.prog.emit(Instr::new(Opcode::Column, cursor, i, r));
        }

        for assign in &stmt.assignments {
            let val = self.compile_expr(&assign.value, Some(&ctx))?;
            for col_name in &assign.columns {
                let col_idx = table.columns.iter()
                    .position(|c| c.name.eq_ignore_ascii_case(col_name))
                    .ok_or_else(|| CompileError::new(format!("unknown column '{col_name}'")))?;
                self.prog.emit(Instr::new(Opcode::Copy, val, first_reg + col_idx as i32, 0));
            }
        }

        let rec = self.alloc_reg();
        self.prog.emit(Instr::new(Opcode::MakeRecord, first_reg, ncols, rec));
        self.prog.emit(Instr::new(Opcode::Insert, cursor, rec, rowid_reg));

        if !stmt.returning.is_empty() {
            self.emit_returning(&stmt.returning, cursor, table, first_reg, ncols)?;
        }

        let from_next_addr = self.prog.next_addr() as i32;
        for j in skip_to_next { self.prog.patch_p2(j, from_next_addr); }
        for fc in from_cursors.iter().rev() {
            self.prog.emit(Instr::new(Opcode::Next, *fc, from_loop_start, 0));
        }

        let main_next_addr = self.prog.next_addr();
        self.prog.emit(Instr::new(Opcode::Next, cursor, main_loop_start, 0));
        for from_rewind in &from_inner_rewinds {
            self.prog.patch_p2(*from_rewind, main_next_addr as i32);
        }

        let halt = self.prog.next_addr();
        self.prog.patch_p2(rewind, halt);
        self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
        Ok(())
    }

    fn compile_delete(&mut self, stmt: &DeleteStmt) -> Result<(), CompileError> {
        if let Some(with_clause) = &stmt.with {
            self.register_cte_schemas(with_clause);
        }

        let tname  = stmt.table.to_ascii_lowercase();
        let table  = self.schema.get_table(&tname)
            .ok_or_else(|| CompileError::new(format!("no such table: {tname}")))?;
        let ncols  = table.columns.len() as i32;
        let cursor = self.alloc_cursor();

        let init_addr = self.prog.emit(Instr::new(Opcode::Init, 0, 0, 0));
        let body = self.prog.next_addr();
        self.prog.patch_p2(init_addr, body);
        self.prog.emit(Instr::new(Opcode::Transaction, 0, 1, 0));
        self.prog.emit(Instr::new(Opcode::OpenWrite, cursor, 0, ncols).p4(P4::Text(tname)));
        let rewind = self.prog.emit(Instr::new(Opcode::Rewind, cursor, 0, 0));

        let loop_start = self.prog.next_addr() as i32;
        let ctx = ScanCtx {
            sources: vec![ScanSource { cursor, table: &table, alias: stmt.alias.clone() }],
        };

        let mut skip_to_next: Vec<usize> = Vec::new();
        if let Some(w) = &stmt.where_ {
            let cond = self.compile_expr(w, Some(&ctx))?;
            let j = self.prog.emit(Instr::new(Opcode::IfNot, cond, 0, 0));
            skip_to_next.push(j);
        }

        // Read columns BEFORE delete for RETURNING.
        let returning_result_regs: Option<Vec<i32>> = if !stmt.returning.is_empty() {
            let first_reg = self.next_reg;
            for i in 0..ncols {
                let r = self.alloc_reg();
                self.prog.emit(Instr::new(Opcode::Column, cursor, i, r));
            }
            let mut result_regs: Vec<i32> = Vec::new();
            for ret_col in &stmt.returning {
                match ret_col {
                    ResultColumn::Expr { expr, .. } => {
                        result_regs.push(self.compile_expr(expr, Some(&ctx))?);
                    }
                    ResultColumn::Star => {
                        for i in 0..ncols { result_regs.push(first_reg + i); }
                    }
                    ResultColumn::TableStar(tbl_name) => {
                        let matches = tbl_name.eq_ignore_ascii_case(&table.name)
                            || stmt.alias.as_deref().map(|a| a.eq_ignore_ascii_case(tbl_name)).unwrap_or(false);
                        if matches {
                            for i in 0..ncols { result_regs.push(first_reg + i); }
                        } else {
                            return Err(CompileError::new(format!("unknown table '{tbl_name}' in RETURNING")));
                        }
                    }
                }
            }
            Some(result_regs)
        } else {
            None
        };

        self.prog.emit(Instr::new(Opcode::Delete, cursor, 0, 0));

        if let Some(result_regs) = returning_result_regs {
            if !result_regs.is_empty() {
                let base  = result_regs[0];
                let count = result_regs.len() as i32;
                self.prog.emit(Instr::new(Opcode::ResultRow, base, count, 0));
            }
        }

        let next_addr = self.prog.emit(Instr::new(Opcode::Next, cursor, loop_start, 0));
        for j in skip_to_next { self.prog.patch_p2(j, next_addr as i32); }

        let halt = self.prog.next_addr();
        self.prog.patch_p2(rewind, halt);
        self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
        Ok(())
    }

    fn compile_pragma(&mut self, stmt: &PragmaStmt) -> Result<(), CompileError> {
        let init_addr = self.prog.emit(Instr::new(Opcode::Init, 0, 0, 0));
        let body = self.prog.next_addr();
        self.prog.patch_p2(init_addr, body);

        if let Some(value) = &stmt.value {
            let key = stmt.name.to_lowercase();
            // Pragma writes are emitted as a no-op here; the actual write
            // happens in try_rust_prepare which has the lock.
            let _ = (key, value);
            self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
        } else {
            let key = stmt.name.to_lowercase();
            if matches!(key.as_str(), "lock_status" | "database_list" | "table_info") {
                self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
                return Ok(());
            }
            let reg = self.alloc_reg();
            let p = self.pragmas;
            match key.as_str() {
                "cache_size"     => { self.prog.emit(Instr::new(Opcode::Integer, p.cache_size      as i32, reg, 0)); }
                "page_size"      => { self.prog.emit(Instr::new(Opcode::Integer, p.page_size       as i32, reg, 0)); }
                "auto_vacuum"    => { self.prog.emit(Instr::new(Opcode::Integer, p.auto_vacuum     as i32, reg, 0)); }
                "user_version"   => { self.prog.emit(Instr::new(Opcode::Integer, p.user_version    as i32, reg, 0)); }
                "application_id" => { self.prog.emit(Instr::new(Opcode::Integer, p.application_id as i32, reg, 0)); }
                "synchronous"    => { self.prog.emit(Instr::new(Opcode::Integer, p.synchronous     as i32, reg, 0)); }
                "query_only"     => { self.prog.emit(Instr::new(Opcode::Integer, if p.query_only { 1 } else { 0 }, reg, 0)); }
                "journal_mode"   => {
                    let mode = match p.journal_mode {
                        JournalMode::Delete   => "delete",
                        JournalMode::Truncate => "truncate",
                        JournalMode::Persist  => "persist",
                        JournalMode::Memory   => "memory",
                        JournalMode::Wal      => "wal",
                        JournalMode::Off      => "off",
                    };
                    self.prog.emit(Instr::new(Opcode::String8, 0, reg, 0).p4(P4::Text(mode.into())));
                }
                "encoding" => { self.prog.emit(Instr::new(Opcode::String8, 0, reg, 0).p4(P4::Text("UTF-8".into()))); }
                _          => { self.prog.emit(Instr::new(Opcode::Integer, 0, reg, 0)); }
            }
            self.prog.emit(Instr::new(Opcode::ResultRow, reg, 1, 0));
            self.prog.emit(Instr::new(Opcode::Halt, 0, 0, 0));
        }
        Ok(())
    }

    fn extract_aggregate_func(expr: &Expr) -> Option<String> {
        match expr {
            Expr::FunctionCall { name, distinct, .. } => {
                let lower = name.to_ascii_lowercase();
                if matches!(lower.as_str(), "count"|"sum"|"avg"|"min"|"max"|"group_concat"|"string_agg") {
                    Some(if *distinct { format!("{lower}#d") } else { lower })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn has_aggregate(columns: &[ResultColumn]) -> bool {
        columns.iter().any(|col| {
            if let ResultColumn::Expr { expr, .. } = col {
                Self::extract_aggregate_func(expr).is_some()
            } else {
                false
            }
        })
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Compile a single parsed [`Stmt`] into a VDBE [`Program`].
///
/// # Locking
/// Acquires `lock_global()` exactly once to snapshot all required global state,
/// then releases it before any compilation or subquery execution begins.
pub fn compile(
    stmt: &Stmt,
    schema: &Schema,
    pragmas: &PragmaState,
    tables: BTreeMap<String, MemTable>,
    indexes: BTreeMap<String, IndexData>,
) -> Result<CompileOutput, CompileError> {
    let mut c = Compiler::new(schema, pragmas, tables, indexes);
    c.compile_stmt(stmt)?;
    Ok(CompileOutput {
        prog: c.prog,
        cte_schemas: c.cte_schemas,
        materialized_tables: c.materialized_tables,
    })
}