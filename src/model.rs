use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Game {
    pub id: u64,
    pub pgn: String,
}

impl Game {
    pub fn from_import(id: u64, imported: import::Game) -> Self {
        Self {
            id,
            pgn: imported.pgn,
        }
    }
}

pub mod import {
    use super::*;

    #[derive(Deserialize)]
    pub struct Game {
        pub pgn: String,
    }
}
