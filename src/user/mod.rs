use bincode::{Encode, Decode};

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct User {
    name: String,
    password: String,
    email: String,
    age: u8,
}
impl User {
    pub fn new(name: String, password: String, email: String, age: u8) -> Self {
        User {
            name,
            password,
            email,
            age,
        }
    }
}
