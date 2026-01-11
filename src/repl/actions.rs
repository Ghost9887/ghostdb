use crate::engine::bitcask::{
    write_to_file,
    read_from_file,
    Bitcask,
};
use crate::user::User;
use std::io::{self, Write};

#[derive(PartialEq)]
pub enum Action {
    Quit,
    Add,
    Get,
    Invalid,
}

pub fn parse_repl_cmd(cmd: &str) -> Action {
    match cmd {
        "add" => Action::Add,
        "get" => Action::Get,
        "q" | "quit" => Action::Quit,
        _ => Action::Invalid,
    }
}

pub fn execute_action(action: Action, engine: &mut Bitcask) -> Result<User, String> {
    match action {
        Action::Add => {
            let (name, password, email, age) = match get_user_info() {
                Ok((n, p, e, a)) => (n, p, e, a),
                Err(_) => {
                    return Err( "Failed to add user".to_string());
                },
            };

            let user = User::new(name, password, email, age);

            match write_to_file(&user, engine) {
                Ok(s) => {
                    println!("{s}");
                    return Ok(user);
                },
                Err(e) => {
                    return Err(e.to_string());
                },
            }
        },
        Action::Get => {
            let id = match get_user_id() {
                Ok(id) => id,
                Err(e) => {
                    return Err(e.to_string());
                },
            };

            let user = match read_from_file(id, engine) {
                Ok(u) => u,
                Err(e) => {
                    return Err(e.to_string());
                },
            };

            Ok(user)
        },
        Action::Invalid => return Err("Invalid command".to_string()),
        _ => return Err("Invalid command".to_string()),
    }
}

fn get_user_info() -> Result<(String, String, String, u8), io::Error>{
    println!("User Info");
    
    print!("Name: ");
    io::stdout().flush()?;
    let mut raw_name = String::new();
    io::stdin().read_line(&mut raw_name)?;
    let name = raw_name.trim().to_string();

    print!("Password: ");
    io::stdout().flush()?;
    let mut raw_pswd = String::new();
    io::stdin().read_line(&mut raw_pswd)?;
    let password = raw_pswd.trim().to_string();

    print!("Email: ");
    io::stdout().flush()?;
    let mut raw_email = String::new();
    io::stdin().read_line(&mut raw_email)?;
    let email = raw_email.trim().to_string();
    
    print!("Age: ");
    io::stdout().flush()?;
    let mut raw_age = String::new();
    io::stdin().read_line(&mut raw_age)?;
    let age = raw_age.trim().parse::<u8>().expect("Age should be between 0-150");
    
    Ok((name, password, email, age))
}

fn get_user_id() -> Result<u64, io::Error> {
    print!("Id: ");
    io::stdout().flush()?;
    let mut raw_id = String::new();
    io::stdin().read_line(&mut raw_id)?;
    let id = raw_id.trim().parse::<u64>().expect("Invalid id");

    Ok(id)
}
