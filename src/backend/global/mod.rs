#[derive(Debug, PartialEq)]
pub enum EngineType {
    Undefined,
    Bitcask,
    BTree,
}

#[derive(Debug, PartialEq)]
pub struct Global {
    engine_type: EngineType,
    active_db: String,
}
impl Global {
    pub fn new() -> Self {
        Global {
            engine_type: EngineType::Undefined,
            active_db: String::new(),
        }
    }
    pub fn change_database(&mut self, name: String) {
        self.active_db = name;
    }
    pub fn change_engine(&mut self, engine_type: EngineType) {
        self.engine_type = engine_type;
    }
    pub fn get_current_database_name(&self) -> String {
        self.active_db.to_string()
    }
}
