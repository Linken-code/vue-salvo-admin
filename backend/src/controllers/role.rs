use crate::controllers::user::PageResponse;
use crate::models::{CreateRole, Permission, PermissionTree, Role};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Debug, serde::Deserialize)]
pub struct RolePermissions {
    pub permission_ids: Vec<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RoleQuery {
    pub name: Option<String>,
    pub code: Option<String>,
}

#[handler]
pub async fn get_roles(req: &mut Request, res: &mut Response) {
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
    match sqlx::query_as::<_, Role>(
        r#"
        INSERT INTO roles (name, code, description)
        VALUES (?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&role.name)
    .bind(&role.code)
    .bind(&role.description)
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
    match sqlx::query_as::<_, Role>(
        r#"
        UPDATE roles 
        SET name = ?, code = ?, description = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(&role.name)
    .bind(&role.code)
    .bind(&role.description)
    .bind(id)
    .fetch_one(pool)
    .await
    {
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
    let role_id = req.param::<i64>("id").unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 查询所有权限
    let permissions = match sqlx::query_as::<_, Permission>(
        r#"
        SELECT * FROM permissions 
        ORDER BY sort, id
        "#,
    )
    .fetch_all(pool)
    .await
    {
        Ok(permissions) => permissions,
        Err(e) => {
            eprintln!("获取权限列表失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "code": 500,
                "message": format!("获取权限列表失败: {}", e)
            })));
            return;
        }
    };

    // 查询角色的权限ID列表
    let role_permission_ids = match sqlx::query_scalar::<_, i64>(
        r#"
        SELECT permission_id FROM role_permissions 
        WHERE role_id = ?
        "#,
    )
    .bind(role_id)
    .fetch_all(pool)
    .await
    {
        Ok(ids) => ids,
        Err(e) => {
            eprintln!("获取角色权限失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "code": 500,
                "message": format!("获取角色权限失败: {}", e)
            })));
            return;
        }
    };

    // 构建权限树
    let mut permission_map: std::collections::HashMap<i32, PermissionTree> =
        std::collections::HashMap::new();
    let mut root_permissions = Vec::new();

    // 首先，将所有权限转换为树节点
    for permission in &permissions {
        let tree_node = PermissionTree {
            id: permission.id,
            name: permission.name.clone(),
            code: permission.code.clone(),
            type_name: permission.type_name.clone(),
            resource: permission.resource.clone(),
            action: permission.action.clone(),
            description: permission.description.clone(),
            children: Vec::new(),
        };

        permission_map.insert(permission.id, tree_node);
    }

    // 然后，构建树结构
    let mut nodes_to_move = Vec::new();
    for permission in &permissions {
        if let Some(parent_id) = permission.parent_id {
            if let Some(node) = permission_map.remove(&permission.id) {
                nodes_to_move.push((parent_id as i32, node));
            }
        }
    }

    // 将节点移动到它们的父节点
    for (parent_id, node) in nodes_to_move {
        if let Some(parent) = permission_map.get_mut(&parent_id) {
            parent.children.push(node);
        }
    }

    // 收集根节点
    for (_, node) in permission_map {
        root_permissions.push(node);
    }

    // 按照 sort 字段排序
    root_permissions.sort_by(|a, b| {
        let a_code_parts: Vec<&str> = a.code.split(':').collect();
        let b_code_parts: Vec<&str> = b.code.split(':').collect();
        a_code_parts
            .len()
            .cmp(&b_code_parts.len())
            .then(a.code.cmp(&b.code))
    });

    // 返回权限树和已选中的权限ID
    res.render(Json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "data": root_permissions,
            "checkedKeys": role_permission_ids
        }
    })));
}

#[handler]
pub async fn update_role_permissions(req: &mut Request, res: &mut Response) {
    let role_id = req.param::<i64>("id").unwrap();
    let role_permissions: RolePermissions = match req.parse_json().await {
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
