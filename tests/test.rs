use ghostdb::engine::bitcask::{
    Bitcask,
    open_file,
    write_to_file,
    read_from_file,
};
use ghostdb::user::User;
use ghostdb::parser::tokens::{Token, tokenize};

#[test]
fn test_open_file() {
    let path = "data/data.log";
    assert!(open_file(path).is_ok());
}   

#[test]
fn test_write_read() {
    let file = open_file("data/data.log").unwrap();
    let mut engine = Bitcask::new(file);
    let user = User::new("Test".to_string(), "test".to_string(), "test@gmail.com".to_string(), 32);

    assert!(write_to_file(&user, &mut engine).is_ok());
    
    let id = 0;
    let new_user = read_from_file(id, &mut engine).unwrap();

    assert_eq!(user, new_user);
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
        Token::Space, 
        Token::RParen, 
        Token::Space,
        Token::Digit(32),
        Token::Delimiter,
        Token::Space,
        Token::Identifier("This is a test".to_string()),
        Token::Space,
        Token::All,
        Token::Space,
        Token::Delimiter,
        Token::Digit(3),
        Token::EOS,
    ];

    assert_eq!(expected_tokens1, tokens1);

    let expected_tokens2: Vec<Token> = vec![
        Token::Select,
        Token::Space,
        Token::All,
        Token::Space,
        Token::From,
        Token::Space,
        Token::Identifier("Users".to_string()),
        Token::EOS,
    ];

    assert_eq!(expected_tokens2, tokens2);

    let expected_tokens3: Vec<Token> = vec![
        Token::Create,
        Token::Space,
        Token::Table,
        Token::Space,
        Token::Identifier("Users".to_string()),
        Token::EOS,
    ];

    assert_eq!(expected_tokens3, tokens3);
}
