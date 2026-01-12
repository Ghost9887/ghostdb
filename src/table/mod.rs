pub enum ColumnType {
    Int,
    Bool,
    Text,
}

pub enum Value {
    Int(i64),
    Bool(bool),
    Text(String),
}

pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
}

pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<Value>>,
}
