mod help;
mod home;

pub use help::Help;
pub use home::Home;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Page {
    Home,
    Help,
}
