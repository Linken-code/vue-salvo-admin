mod config;
mod controllers;
mod models;
mod utils;

use crate::config::database;
use crate::controllers::menu::{create_menu, delete_menu, get_menus, update_menu};
use crate::controllers::permission::{
    create_permission, delete_permission, get_permissions, update_permission,
};
use crate::controllers::role::{
    create_role, delete_role, get_role_permissions, get_roles, update_role, update_role_permissions,
};
use crate::controllers::user::{
    create_user, delete_user, get_current_user, get_user, get_users, login, update_user,
};

use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;

#[handler]
async fn hello() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Hello from Salvo!"
    }))
}

struct DbMiddleware {
    pool: sqlx::SqlitePool,
}

#[async_trait]
impl Handler for DbMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        _res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        req.extensions_mut().insert(self.pool.clone());
        ctrl.call_next(req, _depot, _res).await;
    }
}

#[tokio::main]
async fn main() {
    let pool = match database::init_db().await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to initialize database: {}", err);
            return;
        }
    };

    println!("Server is running at http://localhost:3000");

    let router = Router::new()
        .push(
            Router::with_path("menus")
                .get(get_menus)
                .post(create_menu)
                .push(
                    Router::with_path("<id>")
                        .put(update_menu)
                        .delete(delete_menu),
                ),
        )
        .push(
            Router::with_path("users")
                .get(get_users)
                .post(create_user)
                .push(
                    Router::with_path("<id>")
                        .get(get_user)
                        .put(update_user)
                        .delete(delete_user),
                ),
        )
        .push(
            Router::with_path("roles")
                .get(get_roles)
                .post(create_role)
                .push(
                    Router::with_path("<id>")
                        .put(update_role)
                        .delete(delete_role)
                        .push(
                            Router::with_path("permissions")
                                .get(get_role_permissions)
                                .put(update_role_permissions),
                        ),
                ),
        )
        .push(
            Router::with_path("permissions")
                .get(get_permissions)
                .post(create_permission)
                .push(
                    Router::with_path("<id>")
                        .put(update_permission)
                        .delete(delete_permission),
                ),
        )
        .push(Router::with_path("auth/login").post(login))
        .push(Router::with_path("auth/current-user").get(get_current_user));

    let cors_handler = Cors::new()
        .allow_origin("http://localhost:5173")
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec!["content-type", "authorization"])
        .allow_credentials(true)
        .max_age(3600)
        .into_handler();

    let db_middleware = DbMiddleware { pool };

    let service = Service::new(router).hoop(cors_handler).hoop(db_middleware);

    let acceptor = TcpListener::new("0.0.0.0:3000").bind().await;
    Server::new(acceptor).serve(service).await;
}
