//! Schema types: Schema, Table, Column.

/// A single column definition.
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub declared_type: String,
    pub not_null: bool,
    pub primary_key: bool,
    pub default_value: Option<String>,
}

/// A table definition.
#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub without_rowid: bool,
}

/// Database schema: a collection of tables.
#[derive(Debug, Clone, Default)]
pub struct Schema {
    tables: Vec<Table>,
}

impl Schema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables
            .iter()
            .find(|t| t.name.eq_ignore_ascii_case(name))
    }

    pub fn tables(&self) -> &[Table] {
        &self.tables
    }
}
