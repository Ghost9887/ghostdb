//BITCASK SEARCH ENGINE
use std::{
    fs::{self, File, OpenOptions},
    io,
};
use crate::backend::global::{Global, EngineType};

pub struct Bitcask {
}
impl Bitcask {
    pub fn new() -> Self {
        Bitcask {
        }
    }
} 

pub fn open_file(path: &str) -> Result<File, io::Error>{
    let file: File = OpenOptions::new()
        .read(true)
        .write(true) 
        .open(path)?;

    Ok(file)
}

pub fn execute_create_database(name: String) -> Result<String, String> {
    fs::create_dir(format!("data/{}", name))
        .map_err(|err| format!("Failed to create database: {name}, Error: {err}"))?;

    Ok(format!("Successfully created database: {}", name))
}

pub fn execute_drop_database(name: String, global: &mut Global) -> Result<String, String> {
    fs::remove_dir_all(format!("data/{}", name))
        .map_err(|err| format!("Failed to drop database: {name}, Error: {err}"))?;

    //change the database to undefined if its the current one
    if global.get_current_database_name() == Some(name.as_str()) {
        global.change_database(None);
        global.change_engine(EngineType::Undefined);
    }
    
    Ok(format!("Successfully droped database: {}", name))
}

pub fn execute_change_active_database(name: String, global: &mut Global) -> Result<String, String> {
    //returns a Result<bool>
    let res = fs::exists(format!("data/{}", name)).unwrap();
    match res {
        true => {},
        false => {
            return Err(format!("Failed to change database to: {name}, Error: 'Database doesn't exist'"));
        },
    }

    //make it so we can read the new database data and determine the engine as well
    global.change_database(Some(name.clone()));

    Ok(format!("Successfully changed active database: {}", name))
}

/*
pub fn create_bitcask_database(name: String) -> Result<(), String> {
    fs::create_dir(format!("data/{}", name))
    .map_err(|err| format!("Failed to create database: {name}, {err}"))?;

    Ok(())
}
*/

