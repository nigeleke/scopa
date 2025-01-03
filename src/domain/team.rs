use serde::{Deserialize, Serialize};

use crate::domain::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Engagement {
    Participating,
    Eliminated,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Team {
    id: TeamId,
    name: TeamName,
    engagement: Engagement,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            id: TeamId::new(),
            name: name.into(),
            engagement: Engagement::Participating,
        }
    }

    pub fn id(&self) -> TeamId {
        self.id
    }

    pub fn is_playing(&self) -> bool {
        self.engagement == Engagement::Participating
    }

    pub fn is_not_playing(&self) -> bool {
        !self.is_playing()
    }

    pub fn set_not_playing(&mut self) {
        self.engagement = Engagement::Eliminated
    }

    pub fn name(&self) -> TeamName {
        self.name.clone()
    }
}

pub trait Teams {
    fn teams(&self) -> &[Team];

    fn team_count(&self) -> TeamCount {
        TeamCount::new(self.teams().len())
    }

    fn find_team(&self, id: TeamId) -> Option<&Team> {
        self.teams().iter().find(|team| team.id() == id)
    }
}
