use std::sync::{Arc, Mutex};

use crate::error::Result;
use crate::model::*;

#[derive(Clone, Default)]
pub struct Games {
    games: Arc<Mutex<Vec<Option<Game>>>>,
}

impl Games {
    pub fn len(&self) -> usize {
        let store = self.games.lock().unwrap();

        store.len()
    }

    pub fn find(&self) -> Vec<Game> {
        vec![]
    }

    pub fn insert(&self, game: Game) -> Result<Game> {
        let mut store = self.games.lock().unwrap();

        let id = store.len() as u64;
        let game = Game { id };
        store.push(Some(game.clone()));

        Ok(game)
    }
}

#[test]
fn test_games_repo_default() {
    let games_repo = Games::default();
    assert_eq!(games_repo.len(), 0);
}
