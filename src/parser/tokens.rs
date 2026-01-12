use crate::parser::parse::Action;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    Quit,
    Add,
    Update,
    Select,
    Delete,
    Create,
    Table,
    Database,
    Value(String),
    LParen,
    RParen,
    From,
    All,
    Delimiter,
}

pub fn get_tokens(substrings: Vec<&str>) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    match substrings.get(0) {
        Some(s) => match *s {
            "q" => {
                tokens.push(Token::Quit);
                return Ok(tokens);
            },
            _ => {},
        },
        None => {
            return Ok(tokens);
        },
    }

    for sub in substrings {
        match sub {
            "create" => tokens.push(Token::Create),
            "select" => tokens.push(Token::Select),
            "add" => tokens.push(Token::Add),
            "update" => tokens.push(Token::Update),
            "delete" => tokens.push(Token::Delete),
            "from" => tokens.push(Token::From),
            "database" => tokens.push(Token::Database),
            "table" => tokens.push(Token::Table),
            "*" => tokens.push(Token::All),
            "(" => tokens.push(Token::RParen),
            ")" => tokens.push(Token::LParen),
            "," => tokens.push(Token::Delimiter),
            _ => tokens.push(Token::Value(sub.to_string())),
        }
    }
    Ok(tokens)
}

//checks if the tokens create a valid query
pub fn create_actions(tokens: Vec<Token>) -> Result<Vec<Action>, String> {
    let mut actions: Vec<Action> = Vec::new();
    
    //we get the first token as to know what the query is trying to do 
    let first_token = match tokens.get(0) {
        Some(t) => t,
        None => {
            return Err("No tokens!".to_string());
        },
    };
    
    match first_token {
        Token::Quit =>  {
            return Ok(vec![Action::Quit]);
        },
        Token::Create => {
            
        },
        Token::Update => {

        },
        Token::Add => {

        },
        Token::Delete => {

        },
        _ => {
            actions.push(Action::Invalid);
        },
    }

    Ok(actions)
}
