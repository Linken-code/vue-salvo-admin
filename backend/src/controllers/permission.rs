use salvo::prelude::*;
use sqlx::SqlitePool;

use crate::models::{CreatePermission, Permission, UpdatePermission};

#[handler]
pub async fn get_permissions(req: &mut Request, res: &mut Response) {
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_as::<_, Permission>("SELECT * FROM permissions")
        .fetch_all(pool)
        .await
    {
        Ok(permissions) => {
            res.render(Json(permissions));
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
    let permission: CreatePermission = match req.parse_json().await {
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
        INSERT INTO permissions (name, code, description)
        VALUES (?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&permission.name)
    .bind(&permission.code)
    .bind(&permission.description)
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
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let permission: UpdatePermission = match req.parse_json().await {
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
    let mut query = String::from("UPDATE permissions SET ");
    let mut values = Vec::new();
    let mut params = Vec::new();

    if let Some(name) = permission.name {
        values.push("name = ?");
        params.push(name);
    }
    if let Some(code) = permission.code {
        values.push("code = ?");
        params.push(code);
    }
    if let Some(description) = permission.description {
        values.push("description = ?");
        params.push(description);
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

    let mut query_builder = sqlx::query_as::<_, Permission>(&query);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(id);

    match query_builder.fetch_one(pool).await {
        Ok(permission) => {
            res.render(Json(permission));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to update permission: {}", e)
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
