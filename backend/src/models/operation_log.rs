use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OperationLog {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub module: String,
    pub operation: String,
    pub method: String,
    pub params: Option<String>,
    pub ip: Option<String>,
    pub status: i32,
    pub error: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOperationLog {
    pub user_id: i64,
    pub username: String,
    pub module: String,
    pub operation: String,
    pub method: String,
    pub params: Option<String>,
    pub ip: Option<String>,
    pub status: i32,
    pub error: Option<String>,
}
