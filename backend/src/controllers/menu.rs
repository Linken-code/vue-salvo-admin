use crate::models::Menu;
use salvo::prelude::*;
use sqlx::SqlitePool;

#[handler]
pub async fn get_menus(req: &mut Request, res: &mut Response) {
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_as::<_, Menu>("SELECT * FROM menus ORDER BY sort")
        .fetch_all(pool)
        .await
    {
        Ok(menus) => {
            res.render(Json(menus));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to fetch menus: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn create_menu(req: &mut Request, res: &mut Response) {
    let menu: Menu = match req.parse_json().await {
        Ok(menu) => menu,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid menu data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_as::<_, Menu>(
        r#"
        INSERT INTO menus (parent_id, name, path, component, title, icon, sort, is_hidden)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(menu.parent_id)
    .bind(&menu.name)
    .bind(&menu.path)
    .bind(&menu.component)
    .bind(&menu.title)
    .bind(&menu.icon)
    .bind(menu.sort)
    .bind(menu.is_hidden)
    .fetch_one(pool)
    .await
    {
        Ok(menu) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(menu));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to create menu: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn update_menu(req: &mut Request, res: &mut Response) {
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let menu: Menu = match req.parse_json().await {
        Ok(menu) => menu,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": format!("Invalid menu data: {}", e)
            })));
            return;
        }
    };

    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query_as::<_, Menu>(
        r#"
        UPDATE menus 
        SET parent_id = ?, name = ?, path = ?, component = ?, title = ?, icon = ?, sort = ?, is_hidden = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(menu.parent_id)
    .bind(&menu.name)
    .bind(&menu.path)
    .bind(&menu.component)
    .bind(&menu.title)
    .bind(&menu.icon)
    .bind(menu.sort)
    .bind(menu.is_hidden)
    .bind(id)
    .fetch_one(pool)
    .await
    {
        Ok(menu) => {
            res.render(Json(menu));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": format!("Failed to update menu: {}", e)
            })));
        }
    }
}

#[handler]
pub async fn delete_menu(req: &mut Request, res: &mut Response) {
    let id: i64 = req.param::<String>("id").unwrap().parse().unwrap();
    let pool = req.extensions().get::<SqlitePool>().unwrap();

    match sqlx::query("DELETE FROM menus WHERE id = ?")
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
                "error": format!("Failed to delete menu: {}", e)
            })));
        }
    }
}
