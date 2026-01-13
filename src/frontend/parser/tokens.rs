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
    EOS, //end of statement (;)
    Identifier(String),
    Digit(i64),
    Varchar,
    Int,
    Boolean,
}

pub fn tokenize(cmd: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    
    let mut ip: usize = 0;
    let chars: Vec<char> = cmd.chars().collect();

    while ip < chars.len() {
        let c = match chars.get(ip) {
            Some(c) => c,
            None => break,
        };

        match c {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ',' => tokens.push(Token::Delimiter),
            '*' => tokens.push(Token::All),
            '"' => {
                match parse_identifier(&chars, &mut ip, &mut tokens) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(e);
                    },
                }
            },
            ';' => {
                tokens.push(Token::EOS);
                return Ok(tokens);
            }
            _ => {
                if c.is_numeric() {
                    match parse_digit(&chars, &mut ip, &mut tokens) {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(e);
                        },
                    }          
                }
                else if c.is_alphabetic() {
                    match parse_keyword(&chars, &mut ip, &mut tokens) {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(e);
                        },
                    }
                }
                else if c.is_whitespace() {
                    ip += 1;
                    continue;
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

fn parse_digit(chars: &Vec<char>, ip: &mut usize, tokens: &mut Vec<Token>) -> Result<(), String>{
    let mut digit: String = String::new();
    loop {
        let char = match chars.get(*ip) {
            Some(c) => *c,
            None => break,
        };

        if char == ';' {
            tokens.push(Token::Digit(digit.parse::<i64>().unwrap()));
            *ip -= 1;
            return Ok(());
        }
        else if char.is_numeric() {
            digit.push(char);
        }
        else {
            //make it balance out so we don't miss out on one
            *ip -=1;
            break;
        }
        *ip += 1;
    }
    tokens.push(Token::Digit(digit.parse::<i64>().unwrap()));

    Ok(())
}

fn parse_identifier(chars: &Vec<char>, ip: &mut usize, tokens: &mut Vec<Token>) -> Result<(), String>{
    let mut identifier: String = String::new();
    loop {
        *ip += 1;
        let char = match chars.get(*ip) {
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

    Ok(())
}

fn parse_keyword(chars: &Vec<char>, ip: &mut usize, tokens: &mut Vec<Token>) -> Result<(), String>{
    let mut keyword: String = String::new();
    loop {
        let char = match chars.get(*ip) {
            Some(c) => *c,
            None => break,
        };

        if !char.is_alphabetic() || char == ';'{
            match keyword.as_str() {
                "q" | "quit" => tokens.push(Token::Quit),
                "update" => tokens.push(Token::Update),
                "create" => tokens.push(Token::Create),
                "add" => tokens.push(Token::Add),
                "delete" => tokens.push(Token::Delete),
                "table" => tokens.push(Token::Table),
                "select" => tokens.push(Token::Select),
                "from" => tokens.push(Token::From),
                "database" => tokens.push(Token::Database),
                "varchar" => tokens.push(Token::Varchar),
                "int" => tokens.push(Token::Int),
                "boolean" => tokens.push(Token::Boolean),
                _ => {
                    return Err(format!("Invalid syntax: {keyword}").to_string());
                },
            }
            *ip -= 1;
            break;
        }
        else if char.is_alphabetic() {
            keyword.push(char);
        }
        else {
            break;
        }
        *ip += 1;
    }

    Ok(())
}
