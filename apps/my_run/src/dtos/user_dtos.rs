use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserCreateDto {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserUpdateDto {
    #[validate(length(min = 3))]
    pub username: Option<String>,
    #[validate(length(min = 1))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserQueryDto {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDto {
    pub id: u64,
    pub username: String,
    pub password: String,
}
