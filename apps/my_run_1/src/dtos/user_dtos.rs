use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDTO {
    // pub id: u32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCreateDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UserUpdateDTO {
    pub username: String,
    pub password: String,
}