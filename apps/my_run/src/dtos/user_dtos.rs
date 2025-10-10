use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreateDTO {
    pub name: String,
}

