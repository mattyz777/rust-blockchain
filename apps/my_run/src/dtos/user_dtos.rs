use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct UserDTO {
    pub id: usize,
    pub username: String,
}


#[derive(Clone, Deserialize)]
pub struct UserCreateDTO {
    pub username: String,
}