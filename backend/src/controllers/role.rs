use salvo::prelude::*;
use sqlx::SqlitePool;

use crate::models::{CreateRole, Role, RolePermission, UpdateRole};

#[handler]
pub async fn get_roles(req: &mut Request, res: &mut Response) {
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_as::<_, Role>("SELECT * FROM roles")
        .fetch_all(pool)
        .await
    {
        Ok(roles) => {
            res.render(Json(roles));
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
