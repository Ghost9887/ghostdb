use ghostdb::frontend::parser::parse::{parse_cmd, Statement};
use ghostdb::frontend::parser::ast::{
    CreateStmnt, 
    CreateCore, 
    Identifier, 
    CreateType
};
use ghostdb::frontend::parser::tokens::{Token, tokenize};

#[test]
fn test_tokenization() {
    let cmd1: &str = "( ) 32, \"This is a test\" * ,3;";
    let tokens1: Vec<Token> = tokenize(cmd1).expect("Tokenization should work");

    let cmd2: &str = "select * from \"Users\";";
    let tokens2: Vec<Token> = tokenize(cmd2).expect("Tokenization should work");

    let cmd3: &str = "create table \"Users\";";
    let tokens3: Vec<Token> = tokenize(cmd3).expect("Tokenization should work");

    let expected_tokens1: Vec<Token> = vec![
        Token::LParen, 
        Token::RParen, 
        Token::Digit(32),
        Token::Delimiter,
        Token::Identifier("This is a test".to_string()),
        Token::All,
        Token::Delimiter,
        Token::Digit(3),
        Token::EOS,
    ];

    assert_eq!(expected_tokens1, tokens1);

    let expected_tokens2: Vec<Token> = vec![
        Token::Select,
        Token::All,
        Token::From,
        Token::Identifier("Users".to_string()),
        Token::EOS,
    ];

    assert_eq!(expected_tokens2, tokens2);

    let expected_tokens3: Vec<Token> = vec![
        Token::Create,
        Token::Table,
        Token::Identifier("Users".to_string()),
        Token::EOS,
    ];

    assert_eq!(expected_tokens3, tokens3);
}

#[test]
fn test_run_ast() {

    let expected_statement: Statement = Statement::Create(
        CreateStmnt {
            core: CreateCore {
                create_type: CreateType::Database,
                name: Identifier::Name(String::from("users")),
            },
        }
    );
    
    let query = "create database \"users\";";
    let statement: Statement = parse_cmd(query).unwrap();

    assert_eq!(statement, expected_statement);

}
