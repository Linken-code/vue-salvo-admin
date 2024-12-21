use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Menu {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub component: String,
    pub title: String,
    pub icon: Option<String>,
    pub sort: i32,
    pub is_hidden: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMenu {
    pub parent_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub component: String,
    pub title: String,
    pub icon: Option<String>,
    pub sort: i32,
    pub is_hidden: bool,
} 