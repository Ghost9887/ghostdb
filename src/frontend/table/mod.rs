#[derive(Debug, PartialEq)]
pub enum ColumnType {
    Int,
    Bool,
    Text,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Text(u8),
}

#[derive(Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<Value>>,
}
