use axum::extract::{Path, Query};
use axum::middleware;
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, get_service};

use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::info;
use uuid::Uuid;

use crate::model::ModelController;
use crate::{Error, web};

pub fn app() -> anyhow::Result<axum::Router> {
    let mc = ModelController::new()?;
    let routes_api = web::routes_tickets::routes(mc)
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let router = axum::Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(get_service(ServeDir::new("./")));

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
