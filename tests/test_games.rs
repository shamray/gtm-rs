mod common;

use anyhow::Result;
use axum::http::StatusCode;
use serde_json::json;

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

#[tokio::test]
async fn test_import_game() -> Result<()> {
    let addr = common::spawn_test_server().await?;
    let hc = common::create_test_client(addr)?;

    let pgn = "1. f3 e5 2. g4 Qh4# 0-1";
    let json = json!({"pgn": pgn});

    let response = hc.do_post("/api/games", json).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let game = response.json_body()?;
    assert_eq!(game["pgn"], pgn);

    Ok(())
}

#[tokio::test]
async fn test_imported_game_is_found() -> Result<()> {
    let addr = common::spawn_test_server().await?;
    let hc = common::create_test_client(addr)?;

    let pgn = "1. f3 e5 2. g4 Qh4# 0-1";
    let json = json!({"pgn": pgn});

    let response = hc.do_post("/api/games", json).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let game = response.json_body()?;

    let response = hc.do_get("/api/games").await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json_body()?;
    let games = json.as_array();
    assert_ne!(games, None);

    let games = games.unwrap();

    assert!(games.contains(&game));

    Ok(())
}
