use axum::response::{IntoResponse, Json};
use axum::routing::get;

pub fn routes() -> axum::Router {
    axum::Router::new().route("/healthz", get(healthz))
}

async fn healthz() -> impl IntoResponse {
    Json({})
}
