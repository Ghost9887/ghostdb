use crate::frontend::parser::tokens::{Token, tokenize};
use crate::frontend::actions::Action;
use crate::frontend::parser::ast::run_ast;

pub fn parse_repl_cmd(cmd: String) -> Result<Vec<Action>, String> {
    //tokenize the command
    let tokens: Vec<Token> = match tokenize(&cmd) {
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

    let actions: Vec<Action> = match run_ast(&tokens) {
        Ok(a) => a,
        Err(e) => {
            return Err(e);
        },
    };

    Ok(actions)
}
