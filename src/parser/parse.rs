use crate::parser::tokens::{Token, get_tokens, create_actions};

#[derive(PartialEq, Debug)]
pub enum Action {
    Quit,
    Add,
    Select,
    CreateTable,
    CreateDatabase,
    Delete,
    Update,
    Invalid,
}

pub fn parse_repl_cmd(cmd: String) -> Result<Vec<Action>, String> {
    //split the command into substring
    let substrings: Vec<&str> = cmd.split(' ').collect(); 

    let tokens: Vec<Token> = match get_tokens(substrings) {
        Ok(t) => t,
        Err(e) => {
            return Err(e.to_string());
        },
    };
    println!("Tokens: {:?}", tokens);

    //check wether we have some tokens
    if tokens.is_empty() {
        return Ok(vec![Action::Invalid]);
    }

    let actions: Vec<Action> = match create_actions(tokens) {
        Ok(a) => a,
        Err(e) => {
            return Err(e);
        },
    };

    Ok(actions)
}
