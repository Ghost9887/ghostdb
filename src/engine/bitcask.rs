//BITCASK SEARCH ENGINE
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, SeekFrom, Seek, Write, Read},
    time::{SystemTime, Instant},
};
use bincode::config;
use crate::user::User;
use crate::parser::parse::Action;

pub struct Data {
    pub file_id: u64,
    pub vsz: u64,   
    pub offset: u64,
    pub tmstmp: u64,
}
impl Data {
    fn new(file_id: u64, vsz: u64, offset: u64, tmstmp: u64) -> Self {
        Data {
            file_id,
            vsz,
            offset,
            tmstmp,
        }
    }
}

pub struct Bitcask {
    counter: u64,
    hm: HashMap<u64, Data>,
    active_file: File,
}
impl Bitcask {
    pub fn new(file: File) -> Self {
        Bitcask {
            counter: 0,
            hm: HashMap::new(),
            active_file: file,
        }
    }
    fn insert_entry(&mut self, data: Data) {
        self.hm.insert(self.counter, data);
        self.counter += 1;
    }
    fn get_entry(&self, id: u64) -> Option<&Data> {
        self.hm.get(&id)
    }
}

pub fn open_file(path: &str) -> Result<File, io::Error>{
    let file: File = OpenOptions::new()
        .read(true)
        .write(true) 
        .open(path)?;

    Ok(file)
}

pub fn write_to_file(user: &User, engine: &mut Bitcask) -> Result<String, String> {
    
    let now = Instant::now();

    let config = config::standard(); 

    //serialize the user to bytes
    let bytes: Vec<u8> = bincode::encode_to_vec(user, config).unwrap();
    let offset = engine.active_file.seek(SeekFrom::End(0)).map_err(|_| "Unable to seek in file")?;

    engine.active_file.write_all(&bytes).map_err(|_| "Unable to write to file")?;
    engine.active_file.write_all(b"\n").map_err(|_| "Unable to write to file")?;
    
    //construct a new data struct
    let size = bytes.len() as u64;
    let tmstmp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs() as u64,
        Err(_) => 0,
    };

    let data = Data::new(1, size, offset, tmstmp);
    
    //insert the new data into the hashamp
    engine.insert_entry(data);

    let duration = now.elapsed();
    Ok(format!("Added new entry ({:.2?})", duration).to_string())

}

pub fn read_from_file(id: u64, engine: &mut Bitcask) -> Result<User, String>{
    let (offset, vsz) = match engine.get_entry(id) {
        Some(d) => (d.offset, d.vsz),
        None => {
            return Err("Doesn't exist".to_string());
        },
    };

    let config = config::standard();

    engine.active_file.seek(SeekFrom::Start(offset)).unwrap();

    let mut buffer = vec![0; vsz as usize];
    engine.active_file.read_exact(&mut buffer).unwrap();

    let (user, _): (User, usize) = bincode::decode_from_slice(&buffer, config).unwrap();

    Ok(user)
}

pub fn execute_actions(_actions: &Vec<Action>, _engine: &mut Bitcask) -> Result<(), String> {
    Ok(())
}
