mod game;
mod result;
mod round;
mod state;
mod team;

pub(crate) use game::GameState;
pub(crate) use game::InternalGameState;

pub mod prelude {
    pub use super::game::{Game, GameState};
    pub use super::result::{Error, Result};
    pub use super::round::{Round, Rounds};
    pub use super::state::FinishedState;
    pub use super::state::PlayingState;
    pub use super::state::StartingState;
    pub use super::team::{Team, Teams};
}
