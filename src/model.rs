use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Game {
    pub id: u64,
}
