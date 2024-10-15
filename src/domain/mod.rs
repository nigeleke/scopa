mod game;
mod points;
mod result;
mod round;
mod round_number;
mod state;
mod target;
mod team;
mod team_count;
mod team_id;
mod team_name;

pub(crate) use game::GameState;
pub(crate) use game::InternalGameState;

pub mod prelude {
    pub use super::game::{Game, GameState};
    pub use super::points::Points;
    pub use super::result::{Error, Result};
    pub use super::round::{Round, Rounds};
    pub use super::round_number::RoundNumber;
    pub use super::state::prelude::*;
    pub use super::target::Target;
    pub use super::team::{Team, Teams};
    pub use super::team_count::TeamCount;
    pub use super::team_id::TeamId;
    pub use super::team_name::TeamName;
}
