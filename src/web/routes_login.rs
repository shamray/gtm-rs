use axum::Json;
use axum::routing::post;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::info;

use crate::Error;
use crate::error::Result;

pub fn routes() -> axum::Router {
    axum::Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    info!("{:<12} - api_login", "HANDLER");
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": {"success": true}
    }));

    Ok(body)
}
