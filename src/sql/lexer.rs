/// SQL tokenizer.
///
/// Produces a flat list of `Token`s from a UTF-8 SQL string.
/// Whitespace and comments are silently skipped.

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // ── Literals ──────────────────────────────────────────────────────────────
    Integer(i64),
    Real(f64),
    /// String literal (content without surrounding quotes).
    String(String),
    /// BLOB literal: x'...' (hex bytes).
    Blob(Vec<u8>),
    /// Bind parameter: ?N, :name, @name, $name
    BindParam(String),

    // ── Identifiers / keywords ────────────────────────────────────────────────
    Identifier(String),

    // Keywords
    Abort,
    Action,
    Add,
    After,
    All,
    Alter,
    Always,
    Analyze,
    And,
    As,
    Asc,
    Attach,
    Autoincrement,
    Before,
    Begin,
    Between,
    By,
    Cascade,
    Case,
    Cast,
    Check,
    Collate,
    Column,
    Commit,
    Conflict,
    Constraint,
    Create,
    Cross,
    Current,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
    Database,
    Default,
    Deferrable,
    Deferred,
    Delete,
    Desc,
    Detach,
    Distinct,
    Do,
    Drop,
    Each,
    Else,
    End,
    Escape,
    Except,
    Exclude,
    Exclusive,
    Exists,
    Explain,
    Fail,
    False,
    Filter,
    First,
    Following,
    For,
    Foreign,
    From,
    Full,
    Generated,
    Glob,
    Group,
    Groups,
    Having,
    If,
    Ignore,
    Immediate,
    In,
    Index,
    Indexed,
    Initially,
    Inner,
    Insert,
    Instead,
    Intersect,
    Into,
    Is,
    Isnull,
    Join,
    Key,
    Last,
    Left,
    Like,
    Limit,
    Match,
    Materialized,
    Natural,
    No,
    Not,
    Nothing,
    Notnull,
    Null,
    Nulls,
    Of,
    Offset,
    On,
    Or,
    Order,
    Others,
    Outer,
    Over,
    Partition,
    Plan,
    Pragma,
    Preceding,
    Primary,
    Query,
    Raise,
    Range,
    Recursive,
    References,
    Regexp,
    Reindex,
    Release,
    Rename,
    Replace,
    Restrict,
    Returning,
    Right,
    Rollback,
    Row,
    Rows,
    Savepoint,
    Select,
    Set,
    Table,
    Temp,
    Temporary,
    Then,
    Ties,
    To,
    Transaction,
    Trigger,
    Unbounded,
    Union,
    Unique,
    Update,
    Using,
    Vacuum,
    Values,
    View,
    True,
    Virtual,
    When,
    Where,
    Window,
    With,
    Without,

    // ── Operators and punctuation ─────────────────────────────────────────────
    Star,     // *
    Comma,    // ,
    Dot,      // .
    Semi,     // ;
    LParen,   // (
    RParen,   // )
    Plus,     // +
    Minus,    // -
    Slash,    // /
    Percent,  // %
    Amp,      // &
    Pipe,     // |
    PipePipe, // ||  (string concat)
    Tilde,    // ~
    Eq,       // =
    Ne,       // !=  or  <>
    Lt,       // <
    Le,       // <=
    Gt,       // >
    Ge,       // >=
    LShift,   // <<
    RShift,   // >>
    Question, // ?  (positional bind parameter)
    Colon,    // :  (named bind parameter prefix)
    Dollar,   // $  (named bind parameter prefix)
    At,       // @  (named bind parameter prefix)

    // ── Special ───────────────────────────────────────────────────────────────
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    /// Byte offset of the first character in the source string.
    pub offset: usize,
}

// ── Keyword table ─────────────────────────────────────────────────────────────

fn keyword(s: &str) -> Option<TokenKind> {
    // Case-insensitive match.
    match s.to_ascii_uppercase().as_str() {
        "ABORT" => Some(TokenKind::Abort),
        "ACTION" => Some(TokenKind::Action),
        "ADD" => Some(TokenKind::Add),
        "AFTER" => Some(TokenKind::After),
        "ALL" => Some(TokenKind::All),
        "ALTER" => Some(TokenKind::Alter),
        "ALWAYS" => Some(TokenKind::Always),
        "ANALYZE" => Some(TokenKind::Analyze),
        "AND" => Some(TokenKind::And),
        "AS" => Some(TokenKind::As),
        "ASC" => Some(TokenKind::Asc),
        "ATTACH" => Some(TokenKind::Attach),
        "AUTOINCREMENT" => Some(TokenKind::Autoincrement),
        "BEFORE" => Some(TokenKind::Before),
        "BEGIN" => Some(TokenKind::Begin),
        "BETWEEN" => Some(TokenKind::Between),
        "BY" => Some(TokenKind::By),
        "CASCADE" => Some(TokenKind::Cascade),
        "CASE" => Some(TokenKind::Case),
        "CAST" => Some(TokenKind::Cast),
        "CHECK" => Some(TokenKind::Check),
        "COLLATE" => Some(TokenKind::Collate),
        "COLUMN" => Some(TokenKind::Column),
        "COMMIT" => Some(TokenKind::Commit),
        "CONFLICT" => Some(TokenKind::Conflict),
        "CONSTRAINT" => Some(TokenKind::Constraint),
        "CREATE" => Some(TokenKind::Create),
        "CROSS" => Some(TokenKind::Cross),
        "CURRENT" => Some(TokenKind::Current),
        "CURRENT_DATE" => Some(TokenKind::CurrentDate),
        "CURRENT_TIME" => Some(TokenKind::CurrentTime),
        "CURRENT_TIMESTAMP" => Some(TokenKind::CurrentTimestamp),
        "DATABASE" => Some(TokenKind::Database),
        "DEFAULT" => Some(TokenKind::Default),
        "DEFERRABLE" => Some(TokenKind::Deferrable),
        "DEFERRED" => Some(TokenKind::Deferred),
        "DELETE" => Some(TokenKind::Delete),
        "DESC" => Some(TokenKind::Desc),
        "DETACH" => Some(TokenKind::Detach),
        "DISTINCT" => Some(TokenKind::Distinct),
        "DO" => Some(TokenKind::Do),
        "DROP" => Some(TokenKind::Drop),
        "EACH" => Some(TokenKind::Each),
        "ELSE" => Some(TokenKind::Else),
        "END" => Some(TokenKind::End),
        "ESCAPE" => Some(TokenKind::Escape),
        "EXCEPT" => Some(TokenKind::Except),
        "EXCLUDE" => Some(TokenKind::Exclude),
        "EXCLUSIVE" => Some(TokenKind::Exclusive),
        "EXISTS" => Some(TokenKind::Exists),
        "EXPLAIN" => Some(TokenKind::Explain),
        "FAIL" => Some(TokenKind::Fail),
        "FALSE" => Some(TokenKind::False),
        "FILTER" => Some(TokenKind::Filter),
        "FIRST" => Some(TokenKind::First),
        "FOLLOWING" => Some(TokenKind::Following),
        "FOR" => Some(TokenKind::For),
        "FOREIGN" => Some(TokenKind::Foreign),
        "FROM" => Some(TokenKind::From),
        "FULL" => Some(TokenKind::Full),
        "GENERATED" => Some(TokenKind::Generated),
        "GLOB" => Some(TokenKind::Glob),
        "GROUP" => Some(TokenKind::Group),
        "GROUPS" => Some(TokenKind::Groups),
        "HAVING" => Some(TokenKind::Having),
        "IF" => Some(TokenKind::If),
        "IGNORE" => Some(TokenKind::Ignore),
        "IMMEDIATE" => Some(TokenKind::Immediate),
        "IN" => Some(TokenKind::In),
        "INDEX" => Some(TokenKind::Index),
        "INDEXED" => Some(TokenKind::Indexed),
        "INITIALLY" => Some(TokenKind::Initially),
        "INNER" => Some(TokenKind::Inner),
        "INSERT" => Some(TokenKind::Insert),
        "INSTEAD" => Some(TokenKind::Instead),
        "INTERSECT" => Some(TokenKind::Intersect),
        "INTO" => Some(TokenKind::Into),
        "IS" => Some(TokenKind::Is),
        "ISNULL" => Some(TokenKind::Isnull),
        "JOIN" => Some(TokenKind::Join),
        "KEY" => Some(TokenKind::Key),
        "LAST" => Some(TokenKind::Last),
        "LEFT" => Some(TokenKind::Left),
        "LIKE" => Some(TokenKind::Like),
        "LIMIT" => Some(TokenKind::Limit),
        "MATCH" => Some(TokenKind::Match),
        "MATERIALIZED" => Some(TokenKind::Materialized),
        "NATURAL" => Some(TokenKind::Natural),
        "NO" => Some(TokenKind::No),
        "NOT" => Some(TokenKind::Not),
        "NOTHING" => Some(TokenKind::Nothing),
        "NOTNULL" => Some(TokenKind::Notnull),
        "NULL" => Some(TokenKind::Null),
        "NULLS" => Some(TokenKind::Nulls),
        "OF" => Some(TokenKind::Of),
        "OFFSET" => Some(TokenKind::Offset),
        "ON" => Some(TokenKind::On),
        "OR" => Some(TokenKind::Or),
        "ORDER" => Some(TokenKind::Order),
        "OTHERS" => Some(TokenKind::Others),
        "OUTER" => Some(TokenKind::Outer),
        "OVER" => Some(TokenKind::Over),
        "PARTITION" => Some(TokenKind::Partition),
        "PLAN" => Some(TokenKind::Plan),
        "PRAGMA" => Some(TokenKind::Pragma),
        "PRECEDING" => Some(TokenKind::Preceding),
        "PRIMARY" => Some(TokenKind::Primary),
        "QUERY" => Some(TokenKind::Query),
        "RAISE" => Some(TokenKind::Raise),
        "RANGE" => Some(TokenKind::Range),
        "RECURSIVE" => Some(TokenKind::Recursive),
        "REFERENCES" => Some(TokenKind::References),
        "REGEXP" => Some(TokenKind::Regexp),
        "REINDEX" => Some(TokenKind::Reindex),
        "RELEASE" => Some(TokenKind::Release),
        "RENAME" => Some(TokenKind::Rename),
        "REPLACE" => Some(TokenKind::Replace),
        "RESTRICT" => Some(TokenKind::Restrict),
        "RETURNING" => Some(TokenKind::Returning),
        "RIGHT" => Some(TokenKind::Right),
        "ROLLBACK" => Some(TokenKind::Rollback),
        "ROW" => Some(TokenKind::Row),
        "ROWS" => Some(TokenKind::Rows),
        "SAVEPOINT" => Some(TokenKind::Savepoint),
        "SELECT" => Some(TokenKind::Select),
        "SET" => Some(TokenKind::Set),
        "TABLE" => Some(TokenKind::Table),
        "TEMP" => Some(TokenKind::Temp),
        "TEMPORARY" => Some(TokenKind::Temporary),
        "THEN" => Some(TokenKind::Then),
        "TIES" => Some(TokenKind::Ties),
        "TO" => Some(TokenKind::To),
        "TRANSACTION" => Some(TokenKind::Transaction),
        "TRIGGER" => Some(TokenKind::Trigger),
        "UNBOUNDED" => Some(TokenKind::Unbounded),
        "UNION" => Some(TokenKind::Union),
        "UNIQUE" => Some(TokenKind::Unique),
        "UPDATE" => Some(TokenKind::Update),
        "USING" => Some(TokenKind::Using),
        "VACUUM" => Some(TokenKind::Vacuum),
        "VALUES" => Some(TokenKind::Values),
        "VIEW" => Some(TokenKind::View),
        "TRUE" => Some(TokenKind::True),
        "VIRTUAL" => Some(TokenKind::Virtual),
        "WHEN" => Some(TokenKind::When),
        "WHERE" => Some(TokenKind::Where),
        "WINDOW" => Some(TokenKind::Window),
        "WITH" => Some(TokenKind::With),
        "WITHOUT" => Some(TokenKind::Without),
        _ => None,
    }
}

// ── Lexer ─────────────────────────────────────────────────────────────────────

pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src: src.as_bytes(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn peek2(&self) -> Option<u8> {
        self.src.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let b = self.src.get(self.pos).copied()?;
        self.pos += 1;
        Some(b)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Whitespace
            while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
                self.pos += 1;
            }
            // Line comment: --
            if self.peek() == Some(b'-') && self.peek2() == Some(b'-') {
                self.pos += 2;
                while !matches!(self.peek(), Some(b'\n') | None) {
                    self.pos += 1;
                }
                continue;
            }
            // Block comment: /* ... */
            if self.peek() == Some(b'/') && self.peek2() == Some(b'*') {
                self.pos += 2;
                loop {
                    if self.pos + 1 >= self.src.len() {
                        self.pos = self.src.len();
                        break;
                    }
                    if self.src[self.pos] == b'*' && self.src[self.pos + 1] == b'/' {
                        self.pos += 2;
                        break;
                    }
                    self.pos += 1;
                }
                continue;
            }
            break;
        }
    }

    fn scan_string(&mut self, quote: u8) -> Result<String, String> {
        // Opening quote already consumed. Scan until matching unescaped quote.
        let mut s = String::new();
        loop {
            match self.advance() {
                None => return Err("unterminated string literal".into()),
                Some(c) if c == quote => {
                    // Doubled quote = escaped quote
                    if self.peek() == Some(quote) {
                        self.pos += 1;
                        s.push(c as char);
                    } else {
                        break;
                    }
                }
                Some(c) => s.push(c as char),
            }
        }
        Ok(s)
    }

    fn scan_blob(&mut self) -> Result<Vec<u8>, String> {
        // Expects x' or X' prefix already consumed (the 'x').
        // Advance past the opening quote.
        if self.advance() != Some(b'\'') {
            return Err("expected ' after x in blob literal".into());
        }
        let mut bytes = Vec::new();
        loop {
            let hi = match self.advance() {
                None => return Err("unterminated blob literal".into()),
                Some(b'\'') => break,
                Some(c) => c,
            };
            let lo = match self.advance() {
                None => return Err("unterminated blob literal".into()),
                Some(c) => c,
            };
            let h = hex_digit(hi).ok_or("invalid hex digit in blob")?;
            let l = hex_digit(lo).ok_or("invalid hex digit in blob")?;
            bytes.push((h << 4) | l);
        }
        Ok(bytes)
    }

pub fn next_token(&mut self) -> Result<Token, String> {
    self.skip_whitespace_and_comments();
    let offset = self.pos;

    let b = match self.peek() {
        None => {
            return Ok(Token {
                kind: TokenKind::Eof,
                offset,
            });
        }
        Some(b) => b,
    };

    macro_rules! tok {
        ($kind:expr) => {{
            self.pos += 1;
            Ok(Token {
                kind: $kind,
                offset,
            })
        }};
    }

    match b {
        b'*' => tok!(TokenKind::Star),
        b',' => tok!(TokenKind::Comma),
        b'.' => {
            if self.peek2().is_some_and(|c| c.is_ascii_digit()) {
                self.scan_number(offset)
            } else {
                tok!(TokenKind::Dot)
            }
        }
        b';' => tok!(TokenKind::Semi),
        b'(' => tok!(TokenKind::LParen),
        b')' => tok!(TokenKind::RParen),
        b'+' => tok!(TokenKind::Plus),
        b'%' => tok!(TokenKind::Percent),
        b'&' => tok!(TokenKind::Amp),
        b'~' => tok!(TokenKind::Tilde),
        b'?' => {
            self.pos += 1;
            let start = self.pos;
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.pos += 1;
            }
            let suffix = std::str::from_utf8(&self.src[start..self.pos])
                .map_err(|e| e.to_string())?;
            Ok(Token {
                kind: TokenKind::BindParam(if suffix.is_empty() {
                    "?".into()
                } else {
                    format!("?{}", suffix)
                }),
                offset,
            })
        }
        b'$' | b':' | b'@' => {
            let prefix = b as char;
            self.pos += 1;
            let start = self.pos;
            while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == b'_') {
                self.pos += 1;
            }
            let name = std::str::from_utf8(&self.src[start..self.pos])
                .map_err(|e| e.to_string())?;
            Ok(Token {
                kind: TokenKind::BindParam(format!("{}{}", prefix, name)),
                offset,
            })
        }
        b'-' => tok!(TokenKind::Minus),
        b'/' => tok!(TokenKind::Slash),
        b'|' => {
            self.pos += 1;
            if self.peek() == Some(b'|') {
                self.pos += 1;
                Ok(Token { kind: TokenKind::PipePipe, offset })
            } else {
                Ok(Token { kind: TokenKind::Pipe, offset })
            }
        }
        b'=' => {
            self.pos += 1;
            if self.peek() == Some(b'=') {
                self.pos += 1;
            }
            Ok(Token { kind: TokenKind::Eq, offset })
        }
        b'!' => {
            self.pos += 1;
            if self.peek() == Some(b'=') {
                self.pos += 1;
                Ok(Token { kind: TokenKind::Ne, offset })
            } else {
                Err(format!("unexpected character '!' at offset {offset}"))
            }
        }
        b'<' => {
            self.pos += 1;
            match self.peek() {
                Some(b'=') => { self.pos += 1; Ok(Token { kind: TokenKind::Le, offset }) }
                Some(b'<') => { self.pos += 1; Ok(Token { kind: TokenKind::LShift, offset }) }
                Some(b'>') => { self.pos += 1; Ok(Token { kind: TokenKind::Ne, offset }) }
                _ => Ok(Token { kind: TokenKind::Lt, offset }),
            }
        }
        b'>' => {
            self.pos += 1;
            match self.peek() {
                Some(b'=') => { self.pos += 1; Ok(Token { kind: TokenKind::Ge, offset }) }
                Some(b'>') => { self.pos += 1; Ok(Token { kind: TokenKind::RShift, offset }) }
                _ => Ok(Token { kind: TokenKind::Gt, offset }),
            }
        }
        b'\'' => {
            self.pos += 1;
            let s = self.scan_string(b'\'')?;
            Ok(Token { kind: TokenKind::String(s), offset })
        }
        b'"' => {
            self.pos += 1;
            let s = self.scan_string(b'"')?;
            Ok(Token { kind: TokenKind::Identifier(s), offset })
        }
        b'`' => {
            self.pos += 1;
            let s = self.scan_string(b'`')?;
            Ok(Token { kind: TokenKind::Identifier(s), offset })
        }
        b'[' => {
            self.pos += 1;
            let mut s = String::new();
            loop {
                match self.advance() {
                    None => return Err("unterminated bracket identifier".into()),
                    Some(b']') => break,
                    Some(c) => s.push(c as char),
                }
            }
            Ok(Token { kind: TokenKind::Identifier(s), offset })
        }
        b'0'..=b'9' => self.scan_number(offset),
        b'x' | b'X' if self.peek2() == Some(b'\'') => {
            self.pos += 1;
            let bytes = self.scan_blob()?;
            Ok(Token { kind: TokenKind::Blob(bytes), offset })
        }
        c if c.is_ascii_alphabetic() || c == b'_' => {
            let start = self.pos;
            while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == b'_') {
                self.pos += 1;
            }
            let word = std::str::from_utf8(&self.src[start..self.pos])
                .map_err(|e| e.to_string())?;
            let kind = keyword(word).unwrap_or_else(|| TokenKind::Identifier(word.to_string()));
            Ok(Token { kind, offset })
        }
        other => Err(format!(
            "unexpected character {:?} at offset {offset}",
            other as char
        )),
    }
}

    fn scan_number(&mut self, offset: usize) -> Result<Token, String> {
        let start = self.pos;
        let mut is_real = false;

        // 0x hex integer
        if self.peek() == Some(b'0') && matches!(self.peek2(), Some(b'x' | b'X')) {
            self.pos += 2;
            while matches!(self.peek(), Some(c) if c.is_ascii_hexdigit()) {
                self.pos += 1;
            }
            let s = std::str::from_utf8(&self.src[start..self.pos]).map_err(|e| e.to_string())?;
            let v = i64::from_str_radix(&s[2..], 16).map_err(|e| e.to_string())?;
            return Ok(Token {
                kind: TokenKind::Integer(v),
                offset,
            });
        }

        // Decimal digits
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.pos += 1;
        }
        // Optional fractional part
        if self.peek() == Some(b'.') {
            is_real = true;
            self.pos += 1;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        // Optional exponent
        if matches!(self.peek(), Some(b'e' | b'E')) {
            is_real = true;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }

        let s = unsafe { std::str::from_utf8_unchecked(&self.src[start..self.pos]) };
        if is_real {
            let v: f64 = s
                .parse()
                .map_err(|e: std::num::ParseFloatError| e.to_string())?;
            Ok(Token {
                kind: TokenKind::Real(v),
                offset,
            })
        } else {
            // Try i64, fall back to u64-as-i64, then float
            let v: i64 = s
                .parse()
                .unwrap_or_else(|_| s.parse::<u64>().map(|u| u as i64).unwrap_or(i64::MAX));
            Ok(Token {
                kind: TokenKind::Integer(v),
                offset,
            })
        }
    }
}

fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Tokenise an entire SQL string into a `Vec<Token>`.
/// Returns an error if an unrecognised character is encountered.
pub fn tokenize(sql: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(sql);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token()?;
        let eof = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if eof {
            break;
        }
    }
    Ok(tokens)
}
