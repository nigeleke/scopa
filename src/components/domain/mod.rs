mod points;
mod round_number;
mod target;
mod team_name;

pub mod prelude {
    pub use super::points::{PointsView, PointsEditor};
    pub use super::round_number::RoundNumberView;
    pub use super::target::{TargetView, TargetEditor};
    pub use super::team_name::{TeamNameView, TeamNameEditor};
}