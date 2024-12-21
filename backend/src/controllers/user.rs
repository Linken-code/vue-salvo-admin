use crate::models::{CreateUser, UpdatePasswordRequest, UpdateUser, User};
use crate::utils::password::{hash_password, verify_password};
use base64::{engine::general_purpose::STANDARD, Engine};
use salvo::prelude::*;
use sqlx::SqlitePool;

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
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => {
            res.render(Json(users));
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
