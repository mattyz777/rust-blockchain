use crate::{
    dtos::user_dto::{CreateUserRequest, UpdateUserRequest},
    models::user_model::User,
    routes,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::user::create_user,
        routes::user::list_users,
        routes::user::get_user,
        routes::user::update_user,
        routes::user::delete_user,
    ),
    components(schemas(User, CreateUserRequest, UpdateUserRequest)),
    tags((name = "User API", description = "User management endpoints"))
)]
pub struct ApiDoc;