#[derive(Debug)]
pub enum ColumnType {
    Int,
    Bool,
    Text,
}

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Text(String),
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<Value>>,
}
