use axum::Json;
use axum::routing::post;
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::{Error, Result, web};

pub fn routes() -> axum::Router {
    axum::Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    info!("{:<12} - api_login", "HANDLER");
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {"success": true}
    }));

    Ok(body)
}
