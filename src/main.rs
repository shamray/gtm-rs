use std::net::SocketAddr;

use axum::routing::get;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let routes = axum::Router::new().route("/hello", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}

async fn hello_world() -> axum::response::Json<Value> {
    axum::response::Json(json!({"response": "Hello, world"}))
}
