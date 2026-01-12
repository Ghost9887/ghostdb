use crate::parser::parse::Action;

#[derive(Debug, PartialEq)]
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
    LParen,
    RParen,
    From,
    All,
    Delimiter,
    Space,
    EOS, //end of statement (;)
    Identifier(String),
    Digit(i64),
}

pub fn tokenize(cmd: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    
    let mut ip: usize = 0;
    let chars: Vec<char> = cmd.chars().collect();
    let mut identifier = String::new();
    let mut digit = String::new();

    while ip < chars.len() {
        let c = match chars.get(ip) {
            Some(c) => c,
            None => break,
        };

        match c {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ' ' => tokens.push(Token::Space),
            ',' => tokens.push(Token::Delimiter),
            '*' => tokens.push(Token::All),
            '"' => {
                loop {
                    ip += 1;
                    let char = match chars.get(ip) {
                        Some(c) => *c,
                        None => {
                            return Err("Missing string terminator".to_string());
                        },
                    };
                    if char == ';' {
                        return Err("Missing string terminator".to_string());
                    }
                    else if char == '"' {
                        break;
                    }
                    identifier.push(char);
                }
                tokens.push(Token::Identifier(identifier.clone()));
                identifier.clear();
            },
            ';' => {
                tokens.push(Token::EOS);
                return Ok(tokens);
            }
            _ => {
                if c.is_numeric() {
                    loop {
                        let char = match chars.get(ip) {
                            Some(c) => *c,
                            None => break,
                        };

                        if char == ';' {
                            tokens.push(Token::Digit(digit.parse::<i64>().unwrap()));
                            tokens.push(Token::EOS);
                            return Ok(tokens);
                        }
                        else if char.is_numeric() {
                            digit.push(char);
                        }
                        else {
                            //make it balance out so we don't miss out on one
                            ip -=1;
                            break;
                        }
                        ip += 1;
                    }
                    tokens.push(Token::Digit(digit.parse::<i64>().unwrap()));
                    digit.clear();
                }
                else {
                    return Err(format!("Inavlid syntax: {}", c).to_string());
                }
            },
        }

        ip += 1;
    }
    
    Err("Missing end of statement: ';'".to_string())
}

//checks if the tokens create a valid query
pub fn create_actions(_tokens: Vec<Token>) -> Result<Vec<Action>, String> {
    let actions: Vec<Action> = Vec::new();
    
    Ok(actions)
}
