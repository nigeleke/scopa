use serde::{Deserialize, Serialize};

use crate::domain::{Finished, Game, Playing, Starting, Target};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Starting(Game<Starting>),
    Playing(Game<Playing>),
    Finished(Game<Finished>),
}

impl From<Target> for State {
    fn from(value: Target) -> Self {
        Self::Starting(Game::from(value))
    }
}

impl From<Game<Starting>> for State {
    fn from(value: Game<Starting>) -> Self {
        Self::Starting(value)
    }
}

impl From<Game<Playing>> for State {
    fn from(value: Game<Playing>) -> Self {
        Self::Playing(value)
    }
}

impl From<Game<Finished>> for State {
    fn from(value: Game<Finished>) -> Self {
        Self::Finished(value)
    }
}
