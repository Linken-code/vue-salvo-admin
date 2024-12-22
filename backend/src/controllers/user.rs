use crate::models::{CreateUser, Permission, Role, UpdateUser, User};
use crate::utils::jwt::generate_token;
use crate::utils::password::{hash_password, verify_password};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Debug, serde::Deserialize)]
pub struct UserRoles {
    pub role_ids: Vec<i64>,
}

#[derive(Debug, serde::Serialize)]
pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserQuery {
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub status: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateProfileRequest {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    // 解析登录请求
    let login_req = match req.parse_json::<LoginRequest>().await {
        Ok(req) => req,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "message": format!("无效的请求数据: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 查询用户
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(&login_req.username)
        .fetch_optional(pool)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Database error: {}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": "服务器内部错误"
            })));
            return;
        }
    };

    match user {
        Some(user) => {
            // 验证密码
            if verify_password(&login_req.password, &user.password) {
                // 生成 token
                let token = generate_token(user.id);

                // 返回登录成功响应
                res.render(Json(json!({
                    "token": token,
                    "user": json!({
                        "id": user.id,
                        "username": user.username,
                        "nickname": user.nickname,
                        "email": user.email,
                        "avatar": user.avatar,
                        "status": user.status,
                        "created_at": user.created_at,
                        "updated_at": user.updated_at
                    })
                })));
            } else {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(json!({
                    "message": "用户名或密码错误"
                })));
            }
        }
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(json!({
                "message": "用户名或密码错误"
            })));
        }
    }
}

#[handler]
pub async fn update_password(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = depot.get::<i64>("user_id").unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap().clone();

    match req.parse_json::<UpdatePasswordRequest>().await {
        Ok(pwd_req) => {
            // 先验证旧密码
            let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_one(&pool)
                .await;

            match user {
                Ok(user) => {
                    if !verify_password(&pwd_req.old_password, &user.password) {
                        res.status_code(StatusCode::BAD_REQUEST);
                        res.render(Json(serde_json::json!({
                            "error": "旧密码不正确"
                        })));
                        return;
                    }

                    // 加密新密码
                    let hashed = hash_password(&pwd_req.new_password);

                    // 更新密码
                    let result = sqlx::query("UPDATE users SET password = ? WHERE id = ?")
                        .bind(&hashed)
                        .bind(user_id)
                        .execute(&pool)
                        .await;

                    match result {
                        Ok(_) => {
                            res.render(Json(serde_json::json!({
                                "message": "密码更新成功"
                            })));
                        }
                        Err(e) => {
                            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                            res.render(Json(serde_json::json!({
                                "error": format!("更新失败: {}", e)
                            })));
                        }
                    }
                }
                Err(e) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(serde_json::json!({
                        "error": format!("获取用户信息失败: {}", e)
                    })));
                }
            }
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("无效的请求数据: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn get_users(req: &mut Request, res: &mut Response) {
    let page = req.query::<i64>("page").unwrap_or(1);
    let page_size = req.query::<i64>("page_size").unwrap_or(10);
    let offset = (page - 1) * page_size;

    // 获取查询参数
    let username = req.query::<String>("username").unwrap_or_default();
    let nickname = req.query::<String>("nickname").unwrap_or_default();
    let email = req.query::<String>("email").unwrap_or_default();
    let status = req.query::<i32>("status");

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 构建查询条件
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if !username.is_empty() {
        conditions.push("username LIKE ?");
        params.push(format!("%{}%", username));
    }
    if !nickname.is_empty() {
        conditions.push("nickname LIKE ?");
        params.push(format!("%{}%", nickname));
    }
    if !email.is_empty() {
        conditions.push("email LIKE ?");
        params.push(format!("%{}%", email));
    }
    if let Some(status) = status {
        conditions.push("status = ?");
        params.push(status.to_string());
    }

    // 构建查询语句
    let mut query = String::from("SELECT * FROM users");
    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }
    query.push_str(" LIMIT ? OFFSET ?");

    // 构建计数查询语句
    let mut count_query = String::from("SELECT COUNT(*) FROM users");
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
                "error": format!("Failed to fetch users count: {}", e)
            })));
            return;
        }
    };

    // 获取分页数据
    let mut query_builder = sqlx::query_as::<_, User>(&query);
    for param in &params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(page_size).bind(offset);

    match query_builder.fetch_all(pool).await {
        Ok(users) => {
            let response = PageResponse {
                items: users,
                total,
                page,
                page_size,
            };
            res.render(Json(response));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to fetch users: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn create_user(req: &mut Request, res: &mut Response) {
    let user: CreateUser = match req.parse_json().await {
        Ok(user) => user,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid user data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();
    let hashed_password = hash_password(&user.password);
    match sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, password, nickname, email, avatar, status)
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&user.username)
    .bind(&hashed_password)
    .bind(&user.nickname)
    .bind(&user.email)
    .bind(&user.avatar)
    .bind(user.status)
    .fetch_one(pool)
    .await
    {
        Ok(user) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(user));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to create user: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn update_user(req: &mut Request, res: &mut Response) {
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let user: UpdateUser = match req.parse_json().await {
        Ok(user) => user,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid user data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();
    match sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET nickname = ?, email = ?, avatar = ?, status = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(&user.nickname)
    .bind(&user.email)
    .bind(&user.avatar)
    .bind(user.status)
    .bind(id)
    .fetch_one(pool)
    .await
    {
        Ok(user) => {
            res.render(Json(user));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to update user: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn delete_user(req: &mut Request, res: &mut Response) {
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query("DELETE FROM users WHERE id = ?")
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
                "error": format!("Failed to delete user: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(user)) => {
            res.render(Json(user));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "User not found"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to fetch user: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn get_current_user(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = depot.get::<i64>("user_id").unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 查询用户基本信息
    let user = match sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE id = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    {
        Ok(user) => user,
        Err(e) => {
            eprintln!("获取用户信息失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "code": 500,
                "message": format!("获取用户信息失败: {}", e)
            })));
            return;
        }
    };

    // 查询用户的角色
    let roles = match sqlx::query_as::<_, Role>(
        r#"
        SELECT r.* FROM roles r
        INNER JOIN user_roles ur ON ur.role_id = r.id
        WHERE ur.user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    {
        Ok(roles) => roles,
        Err(e) => {
            eprintln!("获取用户角色失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "code": 500,
                "message": format!("获取用户角色失败: {}", e)
            })));
            return;
        }
    };

    // 返回用户信息和角色
    res.render(Json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "id": user.id,
            "username": user.username,
            "nickname": user.nickname,
            "email": user.email,
            "avatar": user.avatar,
            "status": user.status,
            "roles": roles,
            "created_at": user.created_at,
            "updated_at": user.updated_at
        }
    })));
}

#[handler]
pub async fn get_user_roles(req: &mut Request, res: &mut Response) {
    let user_id = req.param::<i64>("id").unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_as::<_, Role>(
        r#"
        SELECT r.* FROM roles r
        INNER JOIN user_roles ur ON ur.role_id = r.id
        WHERE ur.user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    {
        Ok(roles) => {
            res.render(Json(json!({
                "roles": roles
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to fetch user roles: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn update_user_roles(req: &mut Request, res: &mut Response) {
    let user_id = req.param::<i64>("id").unwrap();
    let user_roles: UserRoles = match req.parse_json().await {
        Ok(data) => data,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid role data: {}", e)
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

    // 删除现有角色
    if let Err(e) = sqlx::query("DELETE FROM user_roles WHERE user_id = ?")
        .bind(user_id)
        .execute(&mut *tx)
        .await
    {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(serde_json::json!({
            "error": format!("Failed to delete existing roles: {}", e)
        })));
        return;
    }

    // 添加新角色
    for role_id in user_roles.role_ids {
        if let Err(e) = sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES (?, ?)")
            .bind(user_id)
            .bind(role_id)
            .execute(&mut *tx)
            .await
        {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to add new roles: {}", e)
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

#[handler]
pub async fn update_profile(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = depot.get::<i64>("user_id").unwrap();

    // 解析请求数据
    let profile_req = match req.parse_json::<UpdateProfileRequest>().await {
        Ok(req) => req,
        Err(e) => {
            eprintln!("解析请求数据失败: {:?}", e);
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "message": format!("无效的请求数据: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 先获取当前用户信息
    let current_user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            eprintln!("获取用户信息失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("获取用户信息失败: {}", e)
            })));
            return;
        }
    };

    // 验证并准备更新数据
    let nickname = if let Some(ref nickname) = profile_req.nickname {
        if nickname.trim().is_empty() {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "message": "昵称不能为空"
            })));
            return;
        }
        nickname.clone()
    } else {
        current_user.nickname
    };

    let email = if let Some(ref email) = profile_req.email {
        if email.trim().is_empty() {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "message": "邮箱不能为空"
            })));
            return;
        }
        email.clone()
    } else {
        current_user.email
    };

    let avatar = profile_req.avatar.or(current_user.avatar);

    // 执行更新
    let result = sqlx::query(
        "UPDATE users SET nickname = ?, email = ?, avatar = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(&nickname)
    .bind(&email)
    .bind(&avatar)
    .bind(user_id)
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            // 查询更新后的用户信息
            let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_one(pool)
                .await;

            match user {
                Ok(user) => {
                    res.render(Json(json!({
                        "message": "更新成功",
                        "user": json!({
                            "id": user.id,
                            "username": user.username,
                            "nickname": user.nickname,
                            "email": user.email,
                            "avatar": user.avatar,
                            "status": user.status,
                            "created_at": user.created_at,
                            "updated_at": user.updated_at
                        })
                    })));
                }
                Err(e) => {
                    eprintln!("获取更新后的用户信息失败: {:?}", e);
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(Json(json!({
                        "message": format!("获取更新后的用户信息失败: {}", e)
                    })));
                }
            }
        }
        Err(e) => {
            eprintln!("更新用户信息失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "message": format!("更新失败: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn get_user_permissions(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = depot.get::<i64>("user_id").unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    // 查询用户的角色
    let roles = match sqlx::query_as::<_, Role>(
        r#"
        SELECT r.* FROM roles r
        INNER JOIN user_roles ur ON ur.role_id = r.id
        WHERE ur.user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    {
        Ok(roles) => roles,
        Err(e) => {
            eprintln!("获取用户角色失败: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({
                "code": 500,
                "message": format!("获取用户角色失败: {}", e)
            })));
            return;
        }
    };

    // 获取角色的所有权限
    let mut permissions = Vec::new();
    for role in roles {
        match sqlx::query_as::<_, Permission>(
            r#"
            SELECT p.* FROM permissions p
            INNER JOIN role_permissions rp ON rp.permission_id = p.id
            WHERE rp.role_id = ?
            "#,
        )
        .bind(role.id)
        .fetch_all(pool)
        .await
        {
            Ok(role_permissions) => {
                permissions.extend(role_permissions);
            }
            Err(e) => {
                eprintln!("获取角色权限失败: {:?}", e);
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(json!({
                    "code": 500,
                    "message": format!("获取角色权限失败: {}", e)
                })));
                return;
            }
        }
    }

    // 去重
    permissions.sort_by(|a, b| a.id.cmp(&b.id));
    permissions.dedup_by(|a, b| a.id == b.id);

    // 获取菜单权限
    let menus = permissions
        .iter()
        .filter(|p| p.type_name == "PAGE")
        .cloned()
        .collect::<Vec<_>>();

    // 返回权限和菜单
    res.render(Json(json!({
        "code": 0,
        "message": "success",
        "data": {
            "permissions": permissions,
            "menus": menus
        }
    })));
}
