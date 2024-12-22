use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub color_start: Option<String>,
    pub color_end: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub color_start: Option<String>,
    pub color_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermissionRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub color_start: Option<String>,
    pub color_end: Option<String>,
}
