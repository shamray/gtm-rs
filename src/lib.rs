use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Json};
use axum::routing::get;

use serde::Deserialize;
use serde_json::json;

pub fn app() -> axum::Router {
    axum::Router::new()
        .route("/hello", get(hello_world))
        .route("/hello2/{name}", get(hello_path))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_world(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("world");
    Json(json!({"response": format!("Hello, {name}")}))
}

async fn hello_path(Path(params): Path<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("world");
    Json(json!({"response": format!("Hello, {name}")}))
}
