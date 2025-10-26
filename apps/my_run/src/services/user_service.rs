use crate::dtos::user_dtos::{UserCreateDTO, UserUpdateDTO, UserDTO};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QueryTrait, Set};
use crate::models::user_model::{ActiveModel as UserActiveModel, Column, Entity as UserEntity, Model as UserModel};
use chrono::Utc;

use bytes::Bytes;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use axum::extract::Multipart;

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
        let query = UserEntity::find()
            .filter(Column::IsDeleted.eq(false));

        let sql = query.build(db.get_database_backend()).to_string();
        println!("get_users SQL: {}", sql);

        let users: Vec<UserModel> = query.all(db).await?;

        Ok(users.into_iter().map(Into::into).collect())
    }

    pub async fn get_user(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<UserDTO>> {
        let query = UserEntity::find()
            .filter(Column::Id.eq(id))
            .filter(Column::IsDeleted.eq(false));

        let sql = query.build(db.get_database_backend()).to_string();
        println!("get_user SQL: {}", sql);

        let user: Option<UserModel> = query.one(db).await?;

        Ok(user.map(Into::into)) // user.map(|model| model.into())
    }

    // search with pagination

    pub async fn delete_user(db: &DatabaseConnection, id: i32) -> anyhow::Result<Option<()>> {
        let query = UserEntity::find()
            .filter(Column::Id.eq(id))
            .filter(Column::IsDeleted.eq(false));

        if let Some(user) = query.one(db).await? {
            let sql = UserEntity::delete_many()
                .filter(Column::Id.eq(user.id))

                .filter(Column::IsDeleted.eq(false))
                .build(db.get_database_backend())
                .to_string();
            println!("delete_user SQL: {}", sql);

            user.into_active_model().delete(db).await?;
        }
        
        Ok(None)
    }

    pub async fn update_user(db: &DatabaseConnection, id: i32, dto: UserUpdateDTO) -> anyhow::Result<Option<()>> {
        let query = UserEntity::find()
            .filter(Column::Id.eq(id))
            .filter(Column::IsDeleted.eq(false));

        if let Some(user) = query.one(db).await? {
            let mut active_model = user.into_active_model();
            active_model.username = Set(dto.username);
            active_model.password = Set(dto.password);
            active_model.updated_at = Set(Some(Utc::now()));

            let update_sql = UserEntity::update(active_model.clone())
                .build(db.get_database_backend())
                .to_string();

            println!("update_user UPDATE SQL: {}", update_sql);

            active_model.update(db).await?;
        }

        Ok(None)
    }


    pub async fn upload_file(mut multipart: Multipart) ->  anyhow::Result<Option<()>> {
        let upload_path = std::env::var("UPLOAD_PATH").expect("UPLOAD_PATH must be set in .env");
        println!("upload_file UPLOAD_PATH: {}", upload_path);

        let upload_dir = Path::new(&upload_path);
        println!("upload_file upload_dir: {}", upload_dir.display());

        std::fs::create_dir_all(upload_dir)?;

        while let Some(field) = multipart.next_field().await? {
            let file_name = field
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "file".to_string());

            let data: Bytes = field.bytes().await?;
            
            // merge file path and file name
            let mut path = PathBuf::from(upload_dir);
            path.push(&file_name);

            let mut file = File::create(&path)?;
            file.write_all(&data)?;

            println!("Uploaded file saved to: {}", path.display());
        }

        Ok(None)
    }

    // download
    // search with pagination
}