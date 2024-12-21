use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub status: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub nickname: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub status: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}
