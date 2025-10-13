use crate::dtos::user_dtos::{UserCreateDTO, UserDTO};
use sea_orm::{ActiveModelTrait, Set, DatabaseConnection};
use crate::models::user_model::{ActiveModel as UserActiveModel, Model as UserModel};
use chrono::Utc;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService {}
    }

    // todo
    pub async fn create_user(db: &DatabaseConnection, dto: UserCreateDTO) -> anyhow::Result<UserDTO> {
        let hashed_password = bcrypt::hash(&dto.password, bcrypt::DEFAULT_COST)?;

        let new_user = UserActiveModel {
            username: Set(dto.username),
            password: Set(hashed_password),
            created_at: Set(Utc::now()),
            updated_at: Set(Some(Utc::now())),
            ..Default::default()
        };

        let inserted: UserModel = new_user.insert(db).await?;
        Ok(UserDTO::from(inserted))
    }
}