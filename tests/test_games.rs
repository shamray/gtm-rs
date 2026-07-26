mod common;

use anyhow::Result;
use axum::http::StatusCode;

#[tokio::test]
async fn test_games_empty() -> Result<()> {
    let addr = common::spawn_test_server().await?;
    let hc = common::create_test_client(addr)?;

    let response = hc.do_get("/api/games").await?;

    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json_body()?;
    let games = json.as_array();
    assert_ne!(games, None);

    let games = games.unwrap();
    assert_eq!(0, games.len());

    Ok(())
}
