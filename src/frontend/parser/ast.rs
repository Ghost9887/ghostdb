use crate::frontend::parser::tokens::Token;
use crate::frontend::parser::parse::{Parser, Statement};
use crate::frontend::table::*;

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name(String),
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Table,
    Database,
}

#[derive(Debug, PartialEq)]
pub struct InsertColumns {
    pub columns: Vec<Column>,
}

#[derive(Debug, PartialEq)]
pub struct CreateCore {
    pub create_type: Type,
    pub name: Identifier,
    pub columns: InsertColumns,
}

#[derive(Debug, PartialEq)]
pub struct DropCore {
    pub drop_type: Type,
    pub name: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct CreateStmnt {
    pub core: CreateCore,
}

#[derive(Debug, PartialEq)]
pub struct DropStmnt {
    pub core: DropCore,
}

pub fn run_ast(tokens: Vec<Token>) -> Result<Statement, String> {
    let mut parser: Parser = Parser::new(tokens);
    
    match parser.peek() {
        Some(t) => match t {
            Token::Create => {
                let create_stmnt = CreateStmnt { 
                    core: parse_create_stmnt(&mut parser)?,
                };
                return Ok(Statement::Create(create_stmnt));
            },
            Token::Drop => {
                let drop_stmnt = DropStmnt {
                    core: parse_drop_stmnt(&mut parser)?,
                };
                return Ok(Statement::Drop(drop_stmnt));
            },
            _ => {
                return Err("Invalid syntax".to_string());
            },
        }
        None => {
            return Err("Invalid syntax".to_string());
        },
    }
}

fn parse_create_stmnt(parser: &mut Parser) -> Result<CreateCore, String> {
    parser.advance();

    match parser.peek() {
        Some(t) => match t {
            Token::Database => {
                let core = CreateCore {
                    create_type: Type::Database,
                    name: parse_identifier(parser)?, 
                    columns: InsertColumns{ columns: Vec::new() },
                };

                match parser.peek_next() {
                    Some(t) => match t {
                        Token::EOS => {
                            return Ok(core);
                        },
                        _ => {
                            return Err("Invalid syntax: Expected ';'".to_string());
                        },
                    },
                    None => {
                            return Err("Invalid syntax: Expected ';'".to_string());
                    },
                }
            },
            Token::Table => {
                return Ok(CreateCore {
                    create_type: Type::Table,
                    name: parse_identifier(parser)?,
                    columns: parse_insert_columns(parser)?,
                });
            },
            _ => {
                return Err("Invalid syntax: Expected ['table', 'database']".to_string());
            },
        },
        None => {
            return Err("Invalid syntax: Expected ['table', 'database']".to_string());
        },
    }
}

fn parse_drop_stmnt(parser: &mut Parser) -> Result<DropCore, String> {
    parser.advance();

    match parser.peek() {
        Some(t) => match t {
            Token::Table => {
                todo!()
            },
            Token::Database => {
                let core = DropCore {
                    name: parse_identifier(parser)?,
                    drop_type: Type::Database,
                };
                match parser.peek_next() {
                    Some(t) => match t {
                        Token::EOS => {
                            return Ok(core);
                        },
                        _ => {
                            return Err("Invalid syntax: Expected ';'".to_string());
                        },
                    },
                    None => {
                            return Err("Invalid syntax: Expected ';'".to_string());
                    },
                }
            },
            _ => {
                return Err("Invalid syntax: Expected ['table', 'database']".to_string());
            },
        },
        None => {
            return Err("Invalid syntax: Expected ['table', 'database']".to_string());
        },
    }   
}

fn parse_insert_columns(parser: &mut Parser) -> Result<InsertColumns, String> {
    parser.advance();

    let mut insert_columns: InsertColumns = InsertColumns{
        columns: Vec::new(),
    };
    
    match parser.peek() {
        Some(t) => match t {
            Token::LParen => {
                insert_columns.columns.append(&mut parse_columns(parser, Vec::new())?);
                return Ok(insert_columns);
            },
            Token::EOS => {
                return Ok(insert_columns);
            },
            _ => {
                return Err("Invalid syntax: Expected ['(', ';']".to_string());
            },
        },
        None => {
            return Err("Invalid syntax: Expected ['(', ';']".to_string());
        }
    }
}

fn parse_columns(parser: &mut Parser, mut columns: Vec<Column>) -> Result<Vec<Column>, String> {
    parser.advance();
    println!("{:?}", columns);

    match parser.peek() {
        Some(t) => match t {
            Token::Identifier(name) => {
                columns.push(Column {
                    name: name.clone(),
                    col_type: parse_column_type(parser)?,
                });

                match parser.peek_next() {
                    Some(t) => match t {
                        Token::Delimiter => {
                            parser.advance();
                            return Ok(parse_columns(parser, columns)?);
                        },
                        Token::RParen => {
                            return Ok(columns);
                        },
                        _ => {
                            return Err("Invalid syntax: Expected [')', ',']".to_string());
                        },
                    },
                    None => {
                        return Err("Invalid syntax: Expected [')', ',']".to_string());
                    },
                }
            },         
            _ => {
                return Err("Invalid syntax: Failed to initialize columns".to_string());
            },
        },
        None => {
            return Err("Invalid syntax: Failed to initialize columns".to_string());
        },
    }
}

fn parse_column_type(parser: &mut Parser) -> Result<ColumnType, String> {
    parser.advance();   

    match parser.peek() {
        Some(t) => match t {
            Token::Varchar => Ok(ColumnType::Text),
            Token::Int => Ok(ColumnType::Int),
            Token::Boolean => Ok(ColumnType::Bool),
            _ => {
                return Err("Invalid syntax: Expected 'column_type'".to_string());
            },
        },
        None => {
            return Err("Invalid syntax: Expected 'column_type'".to_string());
        },
    }
}

fn parse_identifier(parser: &mut Parser) -> Result<Identifier, String> {
    parser.advance();

    match parser.peek() {
        Some(t) => match t {
            Token::Identifier(name) => {
                return Ok(Identifier::Name(name.clone()));
            },
            _ => {
                return Err("Invalid syntax: Expected 'name'".to_string());
            }
        },
        None => {
            return Err("Invalid syntax: Expected 'name'".to_string());
        },
    }
}
