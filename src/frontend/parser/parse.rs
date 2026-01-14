use crate::frontend::parser::tokens::{Token, tokenize};
use crate::frontend::parser::ast::{run_ast, CreateStmnt};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Create(CreateStmnt),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    pub fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }
    pub fn advance(&mut self) {
        self.position += 1;
    }
    pub fn expect_next(&self, token: Token) -> bool {
        let next_token = match self.peek_next() {
            Some(t) => t,
            None => {
                return false;
            }
        };

        *next_token == token
    }
}


pub fn parse_cmd(cmd: &str) -> Result<Statement, String> {
    let tokens: Vec<Token> = tokenize(cmd)?;
        
    let statement: Statement = run_ast(tokens)?; 
    
    Ok(statement)
}
