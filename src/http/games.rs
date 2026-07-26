use axum::extract::State;
use axum::response::{IntoResponse, Json};
use axum::routing::get;

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

async fn list_games(State(state): State<AppState>) -> impl IntoResponse {
    let games = state.games_repo.find();
    Json(games)
}

async fn import_game(State(state): State<AppState>) -> impl IntoResponse {
    Json({})
}
