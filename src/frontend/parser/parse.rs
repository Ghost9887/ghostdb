use crate::frontend::parser::tokens::{Token, tokenize};
use crate::frontend::actions::Action;
use crate::frontend::parser::ast::run_ast;

pub fn parse_repl_cmd(cmd: String) -> Result<Vec<Action>, String> {
    //tokenize the command
    let tokens: Vec<Token> = tokenize(&cmd)?;
        
    println!("Tokens: {:?}", tokens);

    //check wether we have some tokens
    if tokens.is_empty() {
        return Ok(vec![Action::Invalid]);
    }

    let actions: Vec<Action> = run_ast(&tokens)?; 

    Ok(actions)
}
