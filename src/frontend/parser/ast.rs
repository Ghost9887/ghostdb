use crate::frontend::parser::tokens::Token;
use crate::frontend::actions::Action;
use crate::frontend::table::*;

enum AstType{
    Create,
}

pub fn run_ast(tokens: &Vec<Token>) -> Result<Vec<Action>, String> {
    let ast_type = match identify_type(tokens) {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        },
    };

    match ast_type {
        AstType::Create => {
            match generate_create_ast(tokens) {
                Ok(actions) => {
                    return Ok(actions);
                },
                Err(e) => {
                    return Err(e);
                },
            }
        },
    }
}

fn generate_create_ast(tokens: &Vec<Token>) -> Result<Vec<Action>, String> {
    let mut actions: Vec<Action> = Vec::new();
    //check the token length should be always the same
    //create, table/database, "name", ;
    if tokens.len() != 4 {
        return Err("Invalid syntax: {too many args for create statement}".to_string());
    }

    let entry = 1;
    let entry_name = 2;

    match &tokens[entry] {
        Token::Database => {
            match &tokens[entry_name] {
                Token::Identifier(name) => {
                    actions.push(Action::CreateDB(name.clone()));
                    return Ok(actions); 
                },
                _ => {
                    return Err("Invalid syntax: {expected 'database_name'}".to_string());
                },
            }
        },
        Token::Table => {
            match &tokens[entry_name] {
                Token::Identifier(name) => {
                    //create table
                    let mut _table: Table = Table {
                        name: name.clone(),
                        columns: Vec::new(),
                        rows: Vec::new(),
                    };
                    return Ok(actions);
                },
                _ => {
                    return Err("Invalid synatx: {expected 'table_name'}".to_string());
                },
            }
        },
        _ => {
            return Err("Invalid syntax: {expected 'table' or 'database'}".to_string());
        },
    }
}

fn identify_type(tokens: &Vec<Token>) -> Result<AstType, String> {
    //determines the type for now
    let first_token = match tokens.get(0) {
        Some(t) => t,
        None => {
            return Err("Trying to identify a empty token list".to_string());
        },
    };

    match first_token {
        Token::Create => {
            return Ok(AstType::Create);
        },
        _ => {},
    }

    Err(format!("Invalid syntax: {:?}", first_token).to_string())
}
