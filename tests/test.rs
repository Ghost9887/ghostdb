use std::fs;
use ghostdb::frontend::parser::parse::{parse_query, Statement};
use ghostdb::frontend::table::*;
use ghostdb::frontend::parser::tokens::{Token, tokenize};
use ghostdb::frontend::parser::ast::{
    CreateStmnt, 
    CreateCore, 
    Identifier, 
    CreateType,
    InsertColumns,
};
use ghostdb::backend::engine::bitcask::{
    execute_create_database,
};

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
fn test_create_database_ast() {
    let expected_statement: Statement = Statement::Create(
        CreateStmnt {
            core: CreateCore {
                create_type: CreateType::Database,
                name: Identifier::Name(String::from("users")),
                columns: InsertColumns {
                    columns: vec![],
                },
            }, 
        },
    );

    let query = "create database \"users\";";
    let statement: Statement = parse_query(query).unwrap();

    assert_eq!(statement, expected_statement);
}

#[test]
fn test_create_table_ast() {

    let expected_statement: Statement = Statement::Create(
        CreateStmnt {
            core: CreateCore {
                create_type: CreateType::Table,
                name: Identifier::Name(String::from("users")),
                columns: InsertColumns {
                    columns: vec![
                        Column {
                            name: String::from("name"),
                            col_type: ColumnType::Text,
                        },
                        Column {
                            name: String::from("age"),
                            col_type: ColumnType::Int,
                        },
                    ],
                },
            },
        }
    );
    
    let query = "create table \"users\" (\"name\" varchar, \"age\" int);";
    let statement: Statement = parse_query(query).unwrap();

    assert_eq!(statement, expected_statement);
}

#[test]
fn test_execute_create_database() {
    let name = String::from("Test");

    assert!(execute_create_database(name).is_ok());
    
    //delete the folder after
    fs::remove_dir("data/Test").unwrap();
}
