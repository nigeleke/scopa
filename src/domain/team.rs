mod id;
mod name;

pub use id::TeamId;
pub use name::TeamName;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct Team {
    id: TeamId,
    name: TeamName,
    is_eliminated: bool,
}

impl Team {
    pub fn new(name: TeamName) -> Self {
        let id = TeamId::new();
        let is_eliminated = false;

        Team {
            id,
            name,
            is_eliminated,
        }
    }

    pub fn id(&self) -> &TeamId {
        &self.id
    }

    pub fn name(&self) -> &TeamName {
        &self.name
    }

    pub fn is_eliminated(&self) -> bool {
        self.is_eliminated
    }

    pub fn eliminate(&mut self) {
        self.is_eliminated = true;
    }

    pub fn reinstate(&mut self) {
        self.is_eliminated = false;
    }
}
