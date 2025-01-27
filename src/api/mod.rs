pub mod github;

pub async fn fallback() -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    (
        axum::http::StatusCode::NOT_FOUND,
        axum::Json(serde_json::json!({
            "status": "not found",
        })),
    )
}
