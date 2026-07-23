mod common;

use anyhow::Result;
use axum::http::StatusCode;

#[tokio::test]
async fn test_health() -> Result<()> {
    let addr = common::spawn_test_server().await?;
    let hc = common::create_test_client(addr)?;

    let response = hc.do_get("/healthz").await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_path_doesnt_exist() -> Result<()> {
    let addr = common::spawn_test_server().await?;
    let hc = common::create_test_client(addr)?;

    let response = hc.do_get("/hell").await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}
