use salvo::prelude::*;
use serde_json::json;
use sqlx::SqlitePool;

use crate::controllers::user::PageResponse;
use crate::models::{CreatePermissionRequest, Permission, UpdatePermissionRequest};

#[handler]
pub async fn get_permissions(req: &mut Request, res: &mut Response) {
    let page = req.query::<i64>("page").unwrap_or(1);
    let page_size = req.query::<i64>("page_size").unwrap_or(10);
    let offset = (page - 1) * page_size;

    // 获取查询参数
    let name = req.query::<String>("name").unwrap_or_default();
    let code = req.query::<String>("code").unwrap_or_default();

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 构建查询条件
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if !name.is_empty() {
        conditions.push("name LIKE ?");
        params.push(format!("%{}%", name));
    }
    if !code.is_empty() {
        conditions.push("code LIKE ?");
        params.push(format!("%{}%", code));
    }

    // 构建查询语句
    let mut query = String::from("SELECT * FROM permissions");
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    query.push_str(" LIMIT ? OFFSET ?");

    // 构建计数查询语句
    let mut count_query = String::from("SELECT COUNT(*) FROM permissions");
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
            res.render(Json(serde_json::json!({
                "error": format!("Failed to fetch permissions count: {}", e)
            })));
            return;
        }
    };

    // 获取分页数据
    let mut query_builder = sqlx::query_as::<_, Permission>(&query);
    for param in &params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(page_size).bind(offset);

    match query_builder.fetch_all(pool).await {
        Ok(permissions) => {
            let response = PageResponse {
                items: permissions,
                total,
                page,
                page_size,
            };
            res.render(Json(response));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to fetch permissions: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn create_permission(req: &mut Request, res: &mut Response) {
    let permission: CreatePermissionRequest = match req.parse_json().await {
        Ok(permission) => permission,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid permission data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();
    match sqlx::query_as::<_, Permission>(
        r#"
        INSERT INTO permissions (name, code, description, color_start, color_end)
        VALUES (?, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&permission.name)
    .bind(&permission.code)
    .bind(&permission.description)
    .bind(&permission.color_start)
    .bind(&permission.color_end)
    .fetch_one(pool)
    .await
    {
        Ok(permission) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(permission));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to create permission: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn update_permission(req: &mut Request, res: &mut Response) {
    let id = req.param::<i64>("id").unwrap();
    let permission = match req.parse_json::<UpdatePermissionRequest>().await {
        Ok(permission) => permission,
        Err(e) => {
            eprintln!("解析权限数据失败: {:?}", e);
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "message": format!("无效的权限数据: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 更新权限
    let result = sqlx::query(
        "UPDATE permissions SET name = ?, code = ?, description = ?, color_start = ?, color_end = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(&permission.name)
    .bind(&permission.code)
    .bind(&permission.description)
    .bind(&permission.color_start)
    .bind(&permission.color_end)
    .bind(id)
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            // 查询更新后的权限
            let updated_permission =
                sqlx::query_as::<_, Permission>("SELECT * FROM permissions WHERE id = ?")
                    .bind(id)
                    .fetch_one(pool)
                    .await;

            match updated_permission {
                Ok(permission) => {
                    res.render(Json(json!({
                        "message": "更新���功",
                        "permission": permission
                    })));
                }
                Err(e) => {
                    eprintln!("获取更新后的权限失败: {:?}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(json!({
                        "message": format!("获取更新后的权限失败: {}", e)
                    })));
                }
            }
        }
        Err(e) => {
            eprintln!("更新权限失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("更新失败: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn delete_permission(req: &mut Request, res: &mut Response) {
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query("DELETE FROM permissions WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
    {
        Ok(_) => {
            res.status_code(StatusCode::NO_CONTENT);
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to delete permission: {}", e)
            })));
        }
    }
}
