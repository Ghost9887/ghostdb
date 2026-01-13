//BITCASK SEARCH ENGINE
use std::{
    fs::{self, File, OpenOptions},
    io,
};

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

//make a folder in data for the new database
pub fn create_bitcask_database(name: String) -> Result<(), String> {
    fs::create_dir(format!("data/{}", name))
    .map_err(|err| format!("Failed to create database: {name}, {err}"))?;

    Ok(())
}

