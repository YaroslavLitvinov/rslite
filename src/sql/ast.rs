//! Abstract Syntax Tree for SQL statements.

// ── Top-level statement ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Select(SelectStmt),
    Insert(InsertStmt),
    Update(UpdateStmt),
    Delete(DeleteStmt),
    CreateTable(CreateTableStmt),
    CreateIndex(CreateIndexStmt),
    DropTable(DropTableStmt),
    DropIndex(DropIndexStmt),
    Begin(Option<TransactionMode>),
    Commit,
    Rollback,
    Savepoint(String),
    Release(String),
    Pragma(PragmaStmt),
    Explain(Box<Stmt>),
    ExplainQueryPlan(Box<Stmt>),
    Attach { file: Expr, schema: String },
    Detach(String),
    Reindex(Option<String>),
    Vacuum(Option<String>),
    Analyze(Option<String>),
    /// Empty input (just whitespace/comments).
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionMode { Deferred, Immediate, Exclusive }

// ── SELECT ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct SelectStmt {
    pub with: Option<WithClause>,
    pub core: SelectCore,
    pub compounds: Vec<(CompoundOp, SelectCore)>,
    pub order_by: Vec<OrderingTerm>,
    pub limit: Option<LimitClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompoundOp { Union, UnionAll, Intersect, Except }

#[derive(Debug, Clone, PartialEq)]
pub struct SelectCore {
    pub distinct: bool,
    pub columns: Vec<ResultColumn>,
    pub from: Option<FromClause>,
    pub where_: Option<Expr>,
    pub group_by: Vec<Expr>,
    pub having: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResultColumn {
    Star,
    TableStar(String),
    Expr { expr: Expr, alias: Option<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FromClause {
    pub items: Vec<TableOrSubquery>,
    pub joins: Vec<JoinClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableOrSubquery {
    Table(TableRef),
    Subquery { select: Box<SelectStmt>, alias: Option<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableRef {
    pub schema: Option<String>,
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: TableOrSubquery,
    pub constraint: Option<JoinConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    Inner, Left, Right, Full, Cross, Natural,
    LeftOuter, RightOuter, FullOuter,
}
// In your ast.rs — verify these exist:
pub struct Join {
    pub join_type: JoinType,
    pub table: TableOrSubquery,
    pub constraint: Option<JoinConstraint>,
}


#[derive(Debug, Clone, PartialEq)]
pub enum JoinConstraint {
    On(Expr),
    Using(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderingTerm {
    pub expr: Expr,
    pub asc: bool,
    pub nulls_first: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LimitClause {
    pub limit: Expr,
    pub offset: Option<Expr>,
}

// ── WITH ───────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct WithClause {
    pub recursive: bool,
    pub ctes: Vec<Cte>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cte {
    pub name: String,
    pub columns: Vec<String>,
    pub body: Box<SelectStmt>,
    pub materialized: Option<bool>,
}

// ── INSERT ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct InsertStmt {
    pub with: Option<WithClause>,
    pub or: Option<ConflictAction>,
    pub schema: Option<String>,
    pub table: String,
    pub alias: Option<String>,
    pub columns: Vec<String>,
    pub source: InsertSource,
    pub returning: Vec<ResultColumn>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InsertSource {
    Values(Vec<Vec<Expr>>),
    Select(Box<SelectStmt>),
    DefaultValues,
}

// ── UPDATE ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateStmt {
    pub with: Option<WithClause>,
    pub or: Option<ConflictAction>,
    pub schema: Option<String>,
    pub table: String,
    pub alias: Option<String>,
    pub assignments: Vec<Assignment>,
    pub from: Option<FromClause>,
    pub where_: Option<Expr>,
    pub returning: Vec<ResultColumn>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub columns: Vec<String>,
    pub value: Expr,
}

// ── DELETE ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStmt {
    pub with: Option<WithClause>,
    pub schema: Option<String>,
    pub table: String,
    pub alias: Option<String>,
    pub where_: Option<Expr>,
    pub returning: Vec<ResultColumn>,
}

// ── CREATE TABLE ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTableStmt {
    pub temporary: bool,
    pub if_not_exists: bool,
    pub schema: Option<String>,
    pub name: String,
    pub body: CreateTableBody,
    pub without_rowid: bool,
    pub strict: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CreateTableBody {
    Columns { columns: Vec<ColumnDef>, table_constraints: Vec<TableConstraint> },
    As(Box<SelectStmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub type_name: Option<TypeName>,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {
    pub name: String,
    pub args: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnConstraint {
    PrimaryKey { asc: bool, conflict: Option<ConflictAction>, autoincrement: bool },
    NotNull { conflict: Option<ConflictAction> },
    Unique { conflict: Option<ConflictAction> },
    Check(Expr),
    Default(Expr),
    Collate(String),
    References(ForeignKeyClause),
    Generated { expr: Expr, stored: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableConstraint {
    PrimaryKey { name: Option<String>, columns: Vec<IndexedColumn>, conflict: Option<ConflictAction> },
    Unique { name: Option<String>, columns: Vec<IndexedColumn>, conflict: Option<ConflictAction> },
    Check { name: Option<String>, expr: Expr },
    ForeignKey { name: Option<String>, columns: Vec<String>, clause: ForeignKeyClause },
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexedColumn {
    pub name: String,
    pub collate: Option<String>,
    pub asc: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForeignKeyClause {
    pub table: String,
    pub columns: Vec<String>,
    pub on_delete: Option<ReferentialAction>,
    pub on_update: Option<ReferentialAction>,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReferentialAction { SetNull, SetDefault, Cascade, Restrict, NoAction }

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictAction { Rollback, Abort, Fail, Ignore, Replace }

// ── CREATE INDEX ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CreateIndexStmt {
    pub unique: bool,
    pub if_not_exists: bool,
    pub schema: Option<String>,
    pub name: String,
    pub table: String,
    pub columns: Vec<IndexedColumn>,
    pub where_: Option<Expr>,
}

// ── DROP ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct DropTableStmt {
    pub if_exists: bool,
    pub schema: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropIndexStmt {
    pub if_exists: bool,
    pub schema: Option<String>,
    pub name: String,
}

// ── PRAGMA ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct PragmaStmt {
    pub schema: Option<String>,
    pub name: String,
    pub value: Option<PragmaValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PragmaValue {
    Integer(i64),
    Real(f64),
    Text(String),
}

// ── Expressions ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Column { schema: Option<String>, table: Option<String>, name: String },
    BinOp { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    UnaryOp { op: UnaryOp, expr: Box<Expr> },
    FunctionCall { name: String, distinct: bool, args: Vec<Expr>, filter: Option<Box<Expr>> },
    Cast { expr: Box<Expr>, type_name: TypeName },
    Collate { expr: Box<Expr>, collation: String },
    Like { not: bool, op: LikeOp, expr: Box<Expr>, pattern: Box<Expr>, escape: Option<Box<Expr>> },
    Is { not: bool, left: Box<Expr>, right: Box<Expr> },
    IsNull { not: bool, expr: Box<Expr> },
    In { not: bool, expr: Box<Expr>, set: InSet },
    Between { not: bool, expr: Box<Expr>, lo: Box<Expr>, hi: Box<Expr> },
    Case { base: Option<Box<Expr>>, branches: Vec<(Expr, Expr)>, else_: Option<Box<Expr>> },
    Exists(Box<SelectStmt>),
    Subquery(Box<SelectStmt>),
    BindParam(BindParam),
    Raise { action: RaiseAction, message: Option<String> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LikeOp { Like, Glob, Regexp, Match }

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
    Null,
    True,
    False,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    BitAnd, BitOr, LShift, RShift,
    Concat,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp { Neg, Pos, Not, BitNot }

#[derive(Debug, Clone, PartialEq)]
pub enum BindParam {
    Positional,
    PositionalN(u32),
    Named(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InSet {
    List(Vec<Expr>),
    Select(Box<SelectStmt>),
    TableName(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RaiseAction { Ignore, Rollback, Abort, Fail }
