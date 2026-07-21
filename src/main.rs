use axum::{response::IntoResponse, routing::get};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let routes = axum::Router::new().route("/hello", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, routes).await?;

    Ok(())
}

async fn hello_world() -> impl IntoResponse {
    axum::response::Json(json!({"response": "Hello, world"}))
}
