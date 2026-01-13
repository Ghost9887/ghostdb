#[derive(PartialEq)]
pub enum Code {
    Uknown,
    Exit,
    Success(String),
    Error(String),
}
