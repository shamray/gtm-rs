use axum::extract::{Path, Query};
use axum::middleware;
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, get_service};

use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use uuid::Uuid;

use crate::{Error, http};

pub fn app() -> anyhow::Result<axum::Router> {
    let router = axum::Router::new()
        .merge(http::health::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());

    Ok(router)
}

async fn main_response_mapper(res: Response) -> Response {
    info!("{:?}", res);

    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error" : {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            info!("{:?}", client_error_body);

            (*status_code, Json(client_error_body)).into_response()
        });

    info!("server log line - {uuid} - Error - {service_error:?}");

    error_response.unwrap_or(res)
}
