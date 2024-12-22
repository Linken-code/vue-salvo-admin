use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, Clone)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub type_name: String,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub sort: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, serde::Serialize)]
pub struct PermissionTree {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub type_name: String,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub description: Option<String>,
    pub children: Vec<PermissionTree>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermission {
    pub name: String,
    pub code: String,
    pub type_name: String,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub parent_id: Option<i32>,
    pub sort: i32,
    pub description: Option<String>,
    pub color_start: Option<String>,
    pub color_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermission {
    pub name: String,
    pub code: String,
    pub type_name: String,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub parent_id: Option<i32>,
    pub sort: i32,
    pub description: Option<String>,
    pub color_start: Option<String>,
    pub color_end: Option<String>,
}
