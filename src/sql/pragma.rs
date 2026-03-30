// src/wip_db/pragma.rs

/// Quickly detect if SQL is a PRAGMA without full parsing.
pub fn is_pragma(sql: &str) -> bool {
    let s = sql.trim_start();
    s.len() >= 6 && s[..6].eq_ignore_ascii_case("pragma")
}

/// Parsed PRAGMA components.
#[derive(Debug)]
pub struct Pragma {
    pub schema: Option<String>,
    pub name:   String,
    pub value:  Option<String>,
}

/// Parse all four PRAGMA forms:
///   PRAGMA name
///   PRAGMA name = value
///   PRAGMA schema.name
///   PRAGMA 'schema'.name          ← quoted schema from .recover
///   PRAGMA 'schema'.name = value
pub fn parse_pragma(sql: &str) -> Option<Pragma> {
    let s = sql.trim();
    let rest = s.get(6..)?.trim_start()
        .trim_end_matches(';').trim_end();

    let (lhs, value) = if let Some(pos) = rest.find('=') {
        let val = rest[pos+1..].trim()
            .trim_matches('\'').trim_matches('"').to_string();
        (rest[..pos].trim_end(), Some(val))
    } else {
        (rest, None)
    };

    let lhs = lhs.trim();

    if let Some(dot) = lhs.find('.') {
        let schema = lhs[..dot].trim()
            .trim_matches('\'').trim_matches('"').to_string();
        let name = lhs[dot+1..].trim().to_string();
        Some(Pragma { schema: Some(schema), name, value })
    } else {
        Some(Pragma { schema: None, name: lhs.to_string(), value })
    }
}