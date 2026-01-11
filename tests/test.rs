use ghostdb::engine::bitcask::{
    Bitcask,
    open_file,
    write_to_file,
    read_from_file,
};
use ghostdb::user::User;

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
