use salvo::prelude::*;
use serde_json::json;
use std::path::Path;
use uuid::Uuid;

#[handler]
pub async fn upload_file(req: &mut Request, res: &mut Response) {
    if let Some(files) = req.files("file").await {
        if let Some(file) = files.first() {
            let name = file.name().unwrap_or("unknown").to_string();
            let ext = Path::new(&name)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("bin");

            let filename = format!("{}.{}", Uuid::new_v4(), ext);
            let filepath = Path::new("uploads").join(&filename);

            if let Err(e) = std::fs::copy(file.path(), &filepath) {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(json!({
                    "message": format!("Failed to save file: {}", e)
                })));
                return;
            }

            res.render(Json(json!({
                "url": format!("/uploads/{}", filename)
            })));
        } else {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "message": "No file uploaded"
            })));
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(json!({
            "message": "No file uploaded"
        })));
    }
}
