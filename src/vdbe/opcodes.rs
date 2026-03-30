//! Bytecode instruction definitions, opcodes, and program structure.

// ── Opcodes ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    // Control
    Init,
    Halt,
    Goto,
    Union,

    // Constant loads  (p2 = destination register)
    Null,
    Integer,
    Int64,
    Real,
    String8,
    Blob,
    Subprogram,
    // Register moves
    Copy,

    // Arithmetic  p1=lhs p2=rhs p3=dst
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    // Bitwise
    BitAnd,
    BitOr,
    ShiftLeft,
    ShiftRight,
    BitNot, // p1=src p2=dst

    // String
    Concat, // p1=lhs p2=rhs p3=dst

    // Comparison  p1=lhs p2=jump_if_true p3=rhs p5=flags
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical
    And,
    Or,
    Not, // p1=src p2=dst

    // NULL tests
    IsNull,  // p1=reg p2=jump_if_null
    NotNull, // p1=reg p2=jump_if_not_null

    // Cursors
    OpenRead,  // p1=cursor p3=ncols p4=Text(table)
    OpenWrite, // p1=cursor p3=ncols p4=Text(table)
    Close,     // p1=cursor
    Rewind,    // p1=cursor p2=jump_if_empty
    Next,      // p1=cursor p2=jump_back_if_more
    Prev,      // p1=cursor p2=jump_back_if_more

    // Row access
    Column, // p1=cursor p2=col_index p3=dst_reg
    Rowid,  // p1=cursor p2=dst_reg

    // Output
    ResultRow, // p1=first_reg p2=count

    // DML
    MakeRecord, // p1=first_reg p2=count p3=dst_reg
    NewRowid,   // p1=cursor p2=dst_reg
    Insert,     // p1=cursor p2=record_reg p3=rowid_reg
    Delete,     // p1=cursor

    // DDL
    DropTable, // p1=if_exists p4=Text(table_name)

    // ── Aggregation ───────────────────────────────────────────────────────────
    /// Initialise aggregate slot p1 for function named in p4.
    AggInit, // p1=slot_reg p4=FuncName
    /// Feed value in reg[p2] into aggregate slot p1.
    AggStep, // p1=slot_reg p2=val_reg
    /// Copy finalised aggregate from slot p1 into reg[p2].
    AggFinal, // p1=slot_reg p2=dst_reg

    // Legacy aliases kept for backwards compat with exec.rs match arms
    AggCount, // p1=counter_reg  (increment by 1 — still used by simple COUNT(*))
    AggValue, // p1=agg_reg p2=dst_reg

    // ── Sorting ────────────────────────────────────────────────────────────────
    /// Append rows to sort buffer.
    SortRow, // p1=first_result_reg p2=n_result p3=first_key_reg p4=Int(n_keys)
    /// Emit all buffered sorted rows, respecting LIMIT(p1) OFFSET(p2).
    SortEmit, // p1=limit(-1=none) p2=offset p3=n_key_cols p4=Text(dir_string)

    // ── LIMIT without ORDER BY ─────────────────────────────────────────────────
    /// Decrement counter at p1; jump to p2 when it reaches 0 (stop).
    /// p3 = offset counter reg (skip rows while p3 > 0, decrement each skip).
    Limit, // p1=limit_reg p2=jump_when_exhausted p3=offset_reg

    // ── Deduplication (DISTINCT) ───────────────────────────────────────────────
    /// Jump to p2 if reg[p1..p1+p3] was already seen in the distinct set.
    DistinctCheck, // p1=first_reg p2=jump_if_dup p3=count

    // Transaction
    Transaction,

    // Bind parameters
    Variable, // p1=var_num(1-based) p2=dst_reg p4=Text(name)

    // Conditional jumps
    If,    // p1=reg p2=jump_if_truthy
    IfNot, // p1=reg p2=jump_if_falsy

    // Function call
    Function, // p1=argc p2=base_reg p3=dst_reg p4=FuncName

    // ── Index operations ──────────────────────────────────────────────────────
    /// Index equality seek.
    /// p1=cursor_slot p2=jump_if_not_found p3=key_reg p4=Text(index_name)
    SeekEq,   // Looks up key in index, sets cursor.index_rowids if found
    /// Advance index cursor to next matching rowid.
    /// p1=cursor_slot p2=jump_back_if_more
    IdxNext,  // Advances through cursor.index_rowids

    Noop,
}

// ── P4 — typed fourth operand ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum P4 {
    None,
    Int(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
    FuncName(String),
    Program(Box<Program>),
}

// ── Instruction ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Instr {
    pub op: Opcode,
    pub p1: i32,
    pub p2: i32,
    pub p3: i32,
    pub p4: P4,
    pub p5: u16,
}

impl Instr {
    pub fn new(op: Opcode, p1: i32, p2: i32, p3: i32) -> Self {
        Instr {
            op,
            p1,
            p2,
            p3,
            p4: P4::None,
            p5: 0,
        }
    }
    pub fn p4(mut self, p4: P4) -> Self {
        self.p4 = p4;
        self
    }
    pub fn p5(mut self, p5: u16) -> Self {
        self.p5 = p5;
        self
    }
}

// ── Program ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub instrs: Vec<Instr>,
    /// Highest bind-parameter index seen (== number of `?` slots needed).
    pub n_vars: u32,
    /// Column names in result set order
    pub column_names: Vec<String>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            instrs: Vec::new(),
            n_vars: 0,
            column_names: Vec::new(),
        }
    }
}

impl Program {
    pub fn emit(&mut self, instr: Instr) -> usize {
        let addr = self.instrs.len();
        self.instrs.push(instr);
        addr
    }
    pub fn next_addr(&self) -> i32 {
        self.instrs.len() as i32
    }
    pub fn patch_p2(&mut self, addr: usize, target: i32) {
        self.instrs[addr].p2 = target;
    }

    /// Number of output columns: the `p2` field of the first ResultRow or SortRow instruction.
    /// ORDER BY queries use SortRow instead of ResultRow, so we check both.
    pub fn column_count(&self) -> usize {
        self.instrs
            .iter()
            .find(|i| i.op == Opcode::ResultRow || i.op == Opcode::SortRow)
            .map(|i| i.p2 as usize)
            .unwrap_or(0)
    }
}
