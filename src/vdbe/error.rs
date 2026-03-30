//! Compilation error types and utilities.

// ── Compile error ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    Unsupported(String),
    Error(String),
}

impl CompileError {
    pub fn new(msg: impl Into<String>) -> Self {
        CompileError::Error(msg.into())
    }
    pub fn unsupported(feat: impl std::fmt::Display) -> Self {
        CompileError::Unsupported(format!("{}", feat))
    }
    pub fn message(&self) -> &str {
        match self {
            CompileError::Unsupported(m) => m,
            CompileError::Error(m) => m,
        }
    }
}
impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Unsupported(m) => write!(f, "unsupported: {}", m),
            CompileError::Error(m) => write!(f, "{}", m),
        }
    }
}
impl std::error::Error for CompileError {}
