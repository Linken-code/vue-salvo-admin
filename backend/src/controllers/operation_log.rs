use salvo::prelude::*;
use serde_json::json;
use sqlx::SqlitePool;

use crate::controllers::user::PageResponse;
use crate::models::OperationLog;

#[handler]
pub async fn get_operation_logs(req: &mut Request, res: &mut Response) {
    let page = req.query::<i64>("page").unwrap_or(1);
    let page_size = req.query::<i64>("page_size").unwrap_or(10);
    let offset = (page - 1) * page_size;

    // 获取查询参数
    let username = req.query::<String>("username").unwrap_or_default();
    let module = req.query::<String>("module").unwrap_or_default();
    let operation = req.query::<String>("operation").unwrap_or_default();
    let status = req.query::<i32>("status");

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 构建查询条件
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    if !username.is_empty() {
        conditions.push("username LIKE ?");
        params.push(format!("%{}%", username));
    }
    if !module.is_empty() {
        conditions.push("module LIKE ?");
        params.push(format!("%{}%", module));
    }
    if !operation.is_empty() {
        conditions.push("operation LIKE ?");
        params.push(format!("%{}%", operation));
    }
    if let Some(status) = status {
        conditions.push("status = ?");
        params.push(status.to_string());
    }

    // 构建查询语句
    let mut query = String::from("SELECT * FROM operation_logs");
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    // 构建计数查询语句
    let mut count_query = String::from("SELECT COUNT(*) FROM operation_logs");
    if !conditions.is_empty() {
        count_query.push_str(" WHERE ");
        count_query.push_str(&conditions.join(" AND "));
    }

    // 获取总数
    let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query);
    for param in &params {
        count_query_builder = count_query_builder.bind(param);
    }

    let total = match count_query_builder.fetch_one(pool).await {
        Ok(total) => total,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("获取日志总数失败: {}", e)
            })));
            return;
        }
    };

    // 获取分页数据
    let mut query_builder = sqlx::query_as::<_, OperationLog>(&query);
    for param in &params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(page_size).bind(offset);

    match query_builder.fetch_all(pool).await {
        Ok(logs) => {
            let response = PageResponse {
                items: logs,
                total,
                page,
                page_size,
            };
            res.render(Json(response));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("获取日志列表失败: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn delete_operation_logs(req: &mut Request, res: &mut Response) {
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query("DELETE FROM operation_logs")
        .execute(pool)
        .await
    {
        Ok(_) => {
            res.render(Json(json!({
                "message": "清空操作日志成功"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("清空操作日志失败: {}", e)
            })));
        }
    }
}
