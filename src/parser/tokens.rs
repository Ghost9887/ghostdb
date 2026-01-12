#[derive(Debug)]
pub enum Token {
    Quit,
    Add,
    Select,
    Create,
}

pub fn get_tokens(substrings: Vec<&str>) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    for sub in substrings {
        match sub {
            "q" | "quit" => tokens.push(Token::Quit),
            "add" => tokens.push(Token::Add), 
            "select" => tokens.push(Token::Select),
            "create" => tokens.push(Token::Create),
            _ => {
                return Err(format!("Invlalid token: '{}'", sub).to_string());
            },
        }
    }

    Ok(tokens)
}
