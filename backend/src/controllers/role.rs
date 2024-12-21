use salvo::prelude::*;
use serde_json::json;
use sqlx::SqlitePool;

use crate::controllers::user::PageResponse;
use crate::models::{CreateRole, Permission, Role};

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
    let id = req.param::<i64>("id").unwrap();
    let role = match req.parse_json::<Role>().await {
        Ok(role) => role,
        Err(e) => {
            eprintln!("解析角色数据失败: {:?}", e);
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "message": format!("无效的角色数据: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 更新角色
    let result = sqlx::query(
        "UPDATE roles SET name = ?, code = ?, description = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(&role.name)
    .bind(&role.code)
    .bind(&role.description)
    .bind(id)
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            // 查询更新后的角色
            let updated_role = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await;

            match updated_role {
                Ok(role) => {
                    res.render(Json(json!({
                        "message": "更新成功",
                        "role": role
                    })));
                }
                Err(e) => {
                    eprintln!("获取更新后的角色失败: {:?}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(json!({
                        "message": format!("获取更新后的角色失败: {}", e)
                    })));
                }
            }
        }
        Err(e) => {
            eprintln!("更新角色失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("更新失败: {}", e)
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
    let role_id = req.param::<i64>("id").unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 查询角色的权限ID列表
    match sqlx::query_scalar::<_, i64>(
        r#"
        SELECT permission_id FROM role_permissions 
        WHERE role_id = ?
        "#,
    )
    .bind(role_id)
    .fetch_all(pool)
    .await
    {
        Ok(permission_ids) => {
            res.render(Json(json!({
                "permissions": permission_ids
            })));
        }
        Err(e) => {
            eprintln!("获取角色权限失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("获取角色权限失败: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn update_role_permissions(req: &mut Request, res: &mut Response) {
    let role_id = req.param::<i64>("id").unwrap();
    let permissions: Vec<i64> = match req.parse_json().await {
        Ok(data) => data,
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
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("开始事务失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("开始事务失败: {}", e)
            })));
            return;
        }
    };

    // 删除现有���限
    if let Err(e) = sqlx::query("DELETE FROM role_permissions WHERE role_id = ?")
        .bind(role_id)
        .execute(&mut *tx)
        .await
    {
        eprintln!("删除现有权限失败: {:?}", e);
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(json!({
            "message": format!("删除现有权限失败: {}", e)
        })));
        return;
    }

    // 添加新权限
    for permission_id in permissions {
        if let Err(e) =
            sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES (?, ?)")
                .bind(role_id)
                .bind(permission_id)
                .execute(&mut *tx)
                .await
        {
            eprintln!("添加新权限失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("添加新权限失败: {}", e)
            })));
            return;
        }
    }

    // 提交事务
    if let Err(e) = tx.commit().await {
        eprintln!("提交事务失败: {:?}", e);
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(json!({
            "message": format!("提交事务失败: {}", e)
        })));
        return;
    }

    res.render(Json(json!({
        "message": "更新角色权限成功"
    })));
}
