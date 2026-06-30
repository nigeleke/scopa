use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Page {
    Help,
    Game,
}

impl Default for Page {
    fn default() -> Self {
        Self::Game
    }
}
