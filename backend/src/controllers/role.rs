use salvo::prelude::*;
use sqlx::SqlitePool;

use crate::controllers::user::PageResponse;
use crate::models::{CreateRole, Role, RolePermission, UpdateRole};

#[handler]
pub async fn get_roles(req: &mut Request, res: &mut Response) {
    let page = req.query::<i64>("page").unwrap_or(1);
    let page_size = req.query::<i64>("page_size").unwrap_or(10);
    let offset = (page - 1) * page_size;

    // 获取查询参数
    let name = req.query::<String>("name").unwrap_or_default();
    let code = req.query::<String>("code").unwrap_or_default();
    let status = req.query::<i32>("status");

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
    if let Some(status) = status {
        conditions.push("status = ?");
        params.push(status.to_string());
    }

    // 构建查询语句
    let mut query = String::from("SELECT * FROM roles");
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    query.push_str(" LIMIT ? OFFSET ?");

    // 构建计数查询语句
    let mut count_query = String::from("SELECT COUNT(*) FROM roles");
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
                "error": format!("Failed to fetch roles count: {}", e)
            })));
            return;
        }
    };

    // 获取分页数据
    let mut query_builder = sqlx::query_as::<_, Role>(&query);
    for param in &params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(page_size).bind(offset);

    match query_builder.fetch_all(pool).await {
        Ok(roles) => {
            let response = PageResponse {
                items: roles,
                total,
                page,
                page_size,
            };
            res.render(Json(response));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to fetch roles: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn create_role(req: &mut Request, res: &mut Response) {
    let role: CreateRole = match req.parse_json().await {
        Ok(role) => role,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid role data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();
    let status = role.status.unwrap_or(1);
    match sqlx::query_as::<_, Role>(
        r#"
        INSERT INTO roles (name, code, description, status)
        VALUES (?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&role.name)
    .bind(&role.code)
    .bind(&role.description)
    .bind(status)
    .fetch_one(pool)
    .await
    {
        Ok(role) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(role));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to create role: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn update_role(req: &mut Request, res: &mut Response) {
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let role: UpdateRole = match req.parse_json().await {
        Ok(role) => role,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid role data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();
    let mut query = String::from("UPDATE roles SET ");
    let mut values = Vec::new();
    let mut params = Vec::new();

    if let Some(name) = role.name {
        values.push("name = ?");
        params.push(name);
    }
    if let Some(code) = role.code {
        values.push("code = ?");
        params.push(code);
    }
    if let Some(description) = role.description {
        values.push("description = ?");
        params.push(description);
    }
    if let Some(status) = role.status {
        values.push("status = ?");
        params.push(status.to_string());
    }

    if values.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(serde_json::json!({
            "error": "No fields to update"
        })));
        return;
    }

    values.push("updated_at = CURRENT_TIMESTAMP");
    query.push_str(&values.join(", "));
    query.push_str(" WHERE id = ? RETURNING *");

    let mut query_builder = sqlx::query_as::<_, Role>(&query);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(id);

    match query_builder.fetch_one(pool).await {
        Ok(role) => {
            res.render(Json(role));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to update role: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn delete_role(req: &mut Request, res: &mut Response) {
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query("DELETE FROM roles WHERE id = ?")
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
                "error": format!("Failed to delete role: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn get_role_permissions(req: &mut Request, res: &mut Response) {
    let role_id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_scalar::<_, i64>(
        "SELECT permission_id FROM role_permissions WHERE role_id = ?",
    )
    .bind(role_id)
    .fetch_all(pool)
    .await
    {
        Ok(permissions) => {
            res.render(Json(permissions));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to fetch role permissions: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn update_role_permissions(req: &mut Request, res: &mut Response) {
    let role_id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let role_permissions: RolePermission = match req.parse_json().await {
        Ok(data) => data,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid permission data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to start transaction: {}", e)
            })));
            return;
        }
    };

    // 删除现有权限
    if let Err(e) = sqlx::query("DELETE FROM role_permissions WHERE role_id = ?")
        .bind(role_id)
        .execute(&mut *tx)
        .await
    {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(serde_json::json!({
            "error": format!("Failed to delete existing permissions: {}", e)
        })));
        return;
    }

    // 添加新权限
    for permission_id in role_permissions.permission_ids {
        if let Err(e) =
            sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES (?, ?)")
                .bind(role_id)
                .bind(permission_id)
                .execute(&mut *tx)
                .await
        {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to add new permissions: {}", e)
            })));
            return;
        }
    }

    if let Err(e) = tx.commit().await {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(serde_json::json!({
            "error": format!("Failed to commit transaction: {}", e)
        })));
        return;
    }

    res.status_code(StatusCode::NO_CONTENT);
}
