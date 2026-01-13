use crate::backend::engine::bitcask::{Bitcask, create_bitcask_database};
use crate::frontend::table::Table;
use crate::frontend::codes::Code;

#[derive(Debug)]
pub enum Action {
    Quit,
    Invalid,
    CreateDB(String),
    CreateTable(Table),
}

//engine driver thingy (sends commands to the engine to execute)
pub fn execute_actions(actions: &Vec<Action>, _engine: &mut Bitcask) -> Code {
    for action in actions {
        match action {
            Action::Quit => {
                return Code::Exit;
            },
            Action::CreateDB(name) => {
                //just make a folder for now
                match create_bitcask_database(name.clone()) {
                    Ok(_) => {
                        return Code::Success(format!("Successfully created a database: {}", name).to_string());
                    }
                    Err(e) => {
                        return Code::Error(format!("Error: {}", e).to_string());
                    }
                }
            },
            _ => {},
        }
    }
    Code::Uknown
}
