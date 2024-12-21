use crate::models::{CreateUser, Role, UpdatePasswordRequest, UpdateUser, User};
use crate::utils::password::{hash_password, verify_password};
use base64::{engine::general_purpose::STANDARD, Engine};
use salvo::prelude::*;
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

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    #[derive(serde::Deserialize)]
    struct LoginForm {
        username: String,
        password: String,
    }

    let login_form: LoginForm = match req.parse_json().await {
        Ok(form) => form,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid login data: {}", e)
            })));
            return;
        }
    };

    let pool = match req.extensions().get::<SqlitePool>() {
        Some(pool) => pool,
        None => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection not available"
            })));
            return;
        }
    };

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ? AND status = 1")
        .bind(&login_form.username)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(user)) => {
            if verify_password(&login_form.password, &user.password) {
                // 生成一个简单的token在实际应用中应该使用JWT）
                let token = format!("{}:{}", user.id, user.username);
                let token = STANDARD.encode(token.as_bytes());

                res.render(Json(serde_json::json!({
                    "token": token,
                    "user": {
                        "id": user.id,
                        "username": user.username,
                        "nickname": user.nickname,
                        "email": user.email,
                        "avatar": user.avatar,
                        "status": user.status
                    }
                })));
            } else {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(serde_json::json!({
                    "error": "Invalid username or password"
                })));
            }
        }
        Ok(None) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "Invalid username or password"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn update_password(req: &mut Request, res: &mut Response) {
    let id = req.param::<i64>("id").unwrap();
    let update_req: UpdatePasswordRequest = match req.parse_json().await {
        Ok(req) => req,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid request data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();
    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(serde_json::json!({
                "error": "User not found"
            })));
            return;
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
            return;
        }
    };

    if !verify_password(&update_req.old_password, &user.password) {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(serde_json::json!({
            "error": "Invalid old password"
        })));
        return;
    }

    let new_password_hash = hash_password(&update_req.new_password);
    match sqlx::query("UPDATE users SET password = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&new_password_hash)
        .bind(id)
        .execute(pool)
        .await
    {
        Ok(_) => {
            res.status_code(StatusCode::OK);
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to update password: {}", e)
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
    match sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, password, nickname, email, avatar, status)
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&user.username)
    .bind(&user.password)
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
pub async fn get_current_user(req: &mut Request, res: &mut Response) {
    let auth_header: &str = match req.header("authorization") {
        Some(header) => header,
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "No authorization header"
            })));
            return;
        }
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(token) => token,
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "Invalid authorization header format"
            })));
            return;
        }
    };

    let decoded = match STANDARD.decode(token) {
        Ok(decoded) => decoded,
        Err(_) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "Invalid token"
            })));
            return;
        }
    };

    let token_str = match String::from_utf8(decoded) {
        Ok(str) => str,
        Err(_) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "Invalid token format"
            })));
            return;
        }
    };

    let parts: Vec<&str> = token_str.split(':').collect();
    if parts.len() != 2 {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(serde_json::json!({
            "error": "Invalid token format"
        })));
        return;
    }

    let user_id: i64 = match parts[0].parse() {
        Ok(id) => id,
        Err(_) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "Invalid token format"
            })));
            return;
        }
    };

    let pool = match req.extensions().get::<SqlitePool>() {
        Some(pool) => pool,
        None => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Database connection not available"
            })));
            return;
        }
    };

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ? AND status = 1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
    {
        Ok(Some(user)) => {
            res.render(Json(serde_json::json!({
                "id": user.id,
                "username": user.username,
                "nickname": user.nickname,
                "email": user.email,
                "avatar": user.avatar,
                "status": user.status
            })));
        }
        Ok(None) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(serde_json::json!({
                "error": "User not found or inactive"
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Database error: {}", e)
            })));
        }
    }
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
            res.render(Json(roles));
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
