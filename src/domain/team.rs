mod id;
mod name;

pub use id::TeamId;
pub use name::TeamName;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    id: TeamId,
    name: TeamName,
    is_playing: bool,
}

impl Team {
    pub fn new(name: TeamName) -> Self {
        let id = TeamId::new();
        let is_playing = true;

        Team {
            id,
            name,
            is_playing,
        }
    }

    pub fn id(&self) -> &TeamId {
        &self.id
    }

    pub fn name(&self) -> &TeamName {
        &self.name
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}
