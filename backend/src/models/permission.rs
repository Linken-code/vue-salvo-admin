use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermission {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermission {
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
}
