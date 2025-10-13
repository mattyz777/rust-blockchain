use serde::{Deserialize, Serialize};
use crate::models::user_model::Model as UserModel;
use chrono::{DateTime, Utc};

#[derive(Clone, Serialize)]
pub struct UserDTO {
    pub id: i64,
    pub username: String,
    // Frontend new Date('2025-10-13T12:34:56Z').toLocaleString()
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<UserModel> for UserDTO {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            username: model.username,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct UserCreateDTO {
    pub username: String,
    pub password: String,
}