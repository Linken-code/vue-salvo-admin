use salvo::prelude::*;
use serde_json::json;
use sqlx::SqlitePool;

use crate::models::CreateOperationLog;

#[handler]
pub async fn operation_log_middleware(
    req: &mut Request,
    depot: &mut Depot,
    _res: &mut Response,
    _ctrl: &mut FlowCtrl,
) {
    // 打印请求路径和方法
    let path = req.uri().path().to_string();
    let method = req.method().to_string();
    println!("[Operation Log] Processing request: {} {}", method, path);

    // 跳过不需要记录日志的接口
    if path.starts_with("/auth/login") || path.starts_with("/operation-logs") || method == "GET" {
        println!("[Operation Log] Skipping request: {} {}", method, path);
        return;
    }

    // 获取用户信息
    let user_id = match depot.get::<i64>("user_id") {
        Ok(id) => *id,
        _ => {
            println!("[Operation Log] Failed to get user_id from depot");
            return;
        }
    };

    let username = match depot.get::<String>("username") {
        Ok(name) => name.clone(),
        _ => {
            println!("[Operation Log] Failed to get username from depot");
            return;
        }
    };

    println!(
        "[Operation Log] User info - id: {}, username: {}",
        user_id, username
    );

    // 获取请求参数
    let params = {
        let mut params_data = json!({
            "query": {},
            "body": null
        });

        // 获取查询参数
        let query_params: Vec<(String, String)> = req
            .queries()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        if !query_params.is_empty() {
            println!("[Operation Log] Query params: {:?}", query_params);
            params_data["query"] = json!(query_params);
        }

        // 获取请求体
        if let Ok(body) = req.parse_json::<serde_json::Value>().await {
            println!("[Operation Log] Request body: {}", body);
            params_data["body"] = body;
        }

        // 如果没有任何参数，返回 None
        if params_data["query"]
            .as_object()
            .map_or(true, |obj| obj.is_empty())
            && params_data["body"].is_null()
        {
            println!("[Operation Log] No parameters found");
            None
        } else {
            let params_str = serde_json::to_string(&params_data).unwrap_or_default();
            println!("[Operation Log] Parameters: {}", params_str);
            Some(params_str)
        }
    };

    // 获取客户端IP
    let ip = Some(req.remote_addr().to_string());
    println!("[Operation Log] Client IP: {:?}", ip);

    // 构建日志记录
    let module = get_module_name(&path);
    let operation = get_operation_name(&method, &path);
    println!(
        "[Operation Log] Module: {}, Operation: {}",
        module, operation
    );

    let log = CreateOperationLog {
        user_id,
        username,
        module,
        operation,
        method,
        params,
        ip,
        status: 0, // 初始状态
        error: None,
    };

    // 保存到depot中,供后续使用
    depot.insert("operation_log", log);
    println!("[Operation Log] Log record created and stored in depot");
}

#[handler]
pub async fn operation_log_after_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
) {
    println!("[Operation Log] Processing after middleware");

    // 获取日志记录
    let mut log = match depot.remove::<CreateOperationLog>("operation_log").ok() {
        Some(log) => log,
        None => {
            println!("[Operation Log] No log record found in depot");
            return;
        }
    };

    // 获取响应状态
    let status = res.status_code.unwrap_or(StatusCode::OK).as_u16() as i32;
    log.status = status;
    println!("[Operation Log] Response status: {}", status);

    // 如果状态码不是2xx，则记录错误信息
    if status < 200 || status >= 300 {
        let error_msg = json!({
            "status": status,
            "message": "请求失败"
        });
        log.error = Some(error_msg.to_string());
        println!("[Operation Log] Error recorded: {}", error_msg);
    }

    // 获取数据库连接并克隆
    let pool = req.extensions().get::<SqlitePool>().unwrap().clone();

    // 异步记录日志
    tokio::spawn(async move {
        println!("[Operation Log] Inserting log record into database");
        let result = sqlx::query(
            r#"
            INSERT INTO operation_logs (
                user_id, username, module, operation, method,
                params, ip, status, error, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#,
        )
        .bind(&log.user_id)
        .bind(&log.username)
        .bind(&log.module)
        .bind(&log.operation)
        .bind(&log.method)
        .bind(&log.params)
        .bind(&log.ip)
        .bind(&log.status)
        .bind(&log.error)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => println!("[Operation Log] Successfully inserted log record"),
            Err(e) => eprintln!("[Operation Log] Failed to insert log record: {}", e),
        }
    });
}

fn get_module_name(path: &str) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() > 1 {
        match parts[1] {
            "users" => "用户管理",
            "roles" => "角色管理",
            "permissions" => "权限管理",
            "menus" => "菜单管理",
            "operation-logs" => "操作日志",
            "profile" => "个人信息",
            _ => "其他",
        }
        .to_string()
    } else {
        "其他".to_string()
    }
}

fn get_operation_name(method: &str, _path: &str) -> String {
    match method {
        "GET" => "查询",
        "POST" => "新增",
        "PUT" => "修改",
        "DELETE" => "删除",
        "PATCH" => "更新",
        _ => "其他",
    }
    .to_string()
}
