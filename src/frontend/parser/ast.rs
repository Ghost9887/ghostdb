use crate::frontend::parser::tokens::Token;
use crate::frontend::parser::parse::{Parser, Statement};

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Name(String),
}

#[derive(Debug, PartialEq)]
pub enum CreateType {
    Table,
    Database,
}

#[derive(Debug, PartialEq)]
pub struct CreateCore {
    pub create_type: CreateType,
    pub name: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct CreateStmnt {
    pub core: CreateCore,
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
                return Ok(CreateCore {
                    create_type: CreateType::Database,
                    name: parse_create_database(parser)?, 
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

fn parse_create_database(parser: &mut Parser) -> Result<Identifier, String> {
    parser.advance();
    
    match parser.peek() {
        Some(t) => match t {
            Token::Identifier(name) => {
                match parser.expect_next(Token::EOS) {
                    true => {
                        return Ok(Identifier::Name(name.clone()));
                    }
                    false => {
                        return Err("Invalid syntax: Expected ';'".to_string());
                    }
                }
            },
            _ => {
                return Err("Invalid syntax: Expected 'database_name'".to_string());
            }
        },
        None => {
            return Err("Invalid syntax: Expected 'database_name'".to_string());
        },
    }
}
