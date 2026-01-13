use std::mem::discriminant;
use std::fs;
use ghostdb::backend::engine::bitcask::{
    Bitcask,
    open_file,
};
use ghostdb::frontend::codes::Code;
use ghostdb::frontend::actions::{Action, execute_actions};
use ghostdb::frontend::parser::parse::parse_repl_cmd;
use ghostdb::frontend::parser::tokens::{Token, tokenize};

#[test]
fn test_open_file() {
    let path = "data/data.log";
    assert!(open_file(path).is_ok());
}   

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
fn test_create_bitcask_db() {
    let mut engine = Bitcask::new();
    let cmd: &str = "create database \"Test\";"; 
 
    let actions: Vec<Action> = parse_repl_cmd(cmd.to_string()).unwrap();

    let code: Code = execute_actions(&actions, &mut engine); 

    assert_eq!(
        discriminant(&code),
        discriminant(&Code::Success(String::new())),
    );

    //delete the directory
    fs::remove_dir_all("data/Test").unwrap();
}
