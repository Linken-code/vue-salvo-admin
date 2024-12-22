use crate::utils::jwt::verify_token;
use salvo::prelude::*;
use serde_json::json;
use sqlx::SqlitePool;

#[handler]
pub async fn auth_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // 跳过登录接口的认证
    if req.uri().path() == "/auth/login" {
        return;
    }

    // 从请求头中获取 token
    let token = match req.header::<String>("Authorization") {
        Some(auth) => {
            if let Some(token) = auth.strip_prefix("Bearer ") {
                token.to_string()
            } else {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render(Json(json!({
                    "message": "无效的认证头"
                })));
                ctrl.skip_rest();
                return;
            }
        }
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(json!({
                "message": "未提供认证信息"
            })));
            ctrl.skip_rest();
            return;
        }
    };

    // 验证 token
    match verify_token(&token) {
        Some(user_id) => {
            // 获取数据库连接
            let pool = req.extensions().get::<SqlitePool>().unwrap();

            // 查询用户信息
            match sqlx::query_scalar::<_, String>("SELECT username FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_one(pool)
                .await
            {
                Ok(username) => {
                    // 存储用户ID和用户名到depot
                    depot.insert("user_id", user_id);
                    depot.insert("username", username);
                }
                Err(_) => {
                    res.status_code(StatusCode::UNAUTHORIZED);
                    res.render(Json(json!({
                        "message": "用户不存在"
                    })));
                    ctrl.skip_rest();
                }
            }
        }
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(json!({
                "message": "无效的认证信息"
            })));
            ctrl.skip_rest();
        }
    }
}
