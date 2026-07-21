use axum::{response::IntoResponse, routing::get};
use serde_json::json;

pub fn app() -> axum::Router {
    axum::Router::new().route("/hello", get(hello_world))
}

async fn hello_world() -> impl IntoResponse {
    axum::response::Json(json!({"response": "Hello, world"}))
}
