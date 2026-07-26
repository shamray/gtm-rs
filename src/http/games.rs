use axum::extract::State;
use axum::response::Json;
use axum::routing::get;

use crate::error::Result;
use crate::model::*;
use crate::repo;

pub fn routes() -> axum::Router {
    let state = AppState::default();
    axum::Router::new()
        .route("/", get(list_games).post(import_game))
        .with_state(state)
}

#[derive(Clone, Default)]
pub struct AppState {
    games_repo: repo::Games,
}

async fn list_games(State(state): State<AppState>) -> Json<Vec<Game>> {
    let games = state.games_repo.find();
    Json(games)
}

async fn import_game(
    State(state): State<AppState>,
    Json(payload): Json<import::Game>,
) -> Result<Json<Game>> {
    let imported_game: import::Game = payload.into();
    let game = state.games_repo.insert(imported_game)?;

    Ok(Json(game))
}
