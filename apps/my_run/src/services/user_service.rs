use crate::dtos::user_dtos::{UserCreateDTO, UserDTO};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::models::user_model::{ActiveModel as UserActiveModel, Column, Entity as UserEntity, Model as UserModel};
use chrono::Utc;

pub struct UserService;

impl UserService {
    pub async fn create_user(db: &DatabaseConnection, dto: UserCreateDTO) -> anyhow::Result<UserDTO> {
        let hashed_password = bcrypt::hash(&dto.password, bcrypt::DEFAULT_COST)?;

        let new_user = UserActiveModel {
            username: Set(dto.username),
            password: Set(hashed_password),
            created_at: Set(Utc::now()),
            updated_at: Set(Some(Utc::now())),
            ..Default::default() // id: NotSet, is_deleted: NotSet
        };

        let inserted: UserModel = new_user.insert(db).await?;
        Ok(UserDTO::from(inserted))
        // Ok(inserted.into())
    }

    pub async fn get_users(db: &DatabaseConnection) -> anyhow::Result<Vec<UserDTO>> {
        let users: Vec<UserModel> = UserEntity::find()
            .filter(Column::IsDeleted.eq(false)) // Requires `use ColumnTrait`
            .all(db)
            .await?;

        Ok(users.into_iter().map(Into::into).collect())
    }
}