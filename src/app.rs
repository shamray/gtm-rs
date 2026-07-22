use axum::extract::{Path, Query};
use axum::middleware;
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, get_service};

use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::info;

use crate::model::ModelController;
use crate::web;

pub fn app() -> anyhow::Result<axum::Router> {
    let mc = ModelController::new()?;
    let router = axum::Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(get_service(ServeDir::new("./")));

    Ok(router)
}

async fn main_response_mapper(res: Response) -> Response {
    info!("{:?}", res);
    res
}

fn routes_hello() -> axum::Router {
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
