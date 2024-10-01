use crate::types::*;

#[derive(Clone, Debug, PartialEq)]
enum Engagement {
    Participating,
    Eliminated
}

#[derive(Clone, Debug, PartialEq)]
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

    pub fn is_active(&self) -> bool {
        self.engagement == Engagement::Participating
    }

    pub fn disengage(&mut self) {
        self.engagement = Engagement::Eliminated
    }

    pub fn name(&self) -> TeamName {
        self.name.clone()
    }
}
