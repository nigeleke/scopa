use crate::types::*;

use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Round {
    scopas: HashMap<TeamId, Points>,
    highest_card_count: Option<TeamId>,
    highest_coins_count: Option<TeamId>,
    settobello: Option<TeamId>,
    premiere: Option<TeamId>,
}

impl Round {
    pub fn with_scopas(mut self, id: &TeamId, count: Points) -> Self {
        self.scopas.insert(*id, count);
        self
    }

    pub fn with_highest_card_count(mut self, id: &TeamId) -> Self {
        self.highest_card_count = Option::Some(*id);
        self
    }

    pub fn with_highest_coins_count(mut self, id: &TeamId) -> Self {
        self.highest_coins_count = Option::Some(*id);
        self
    }

    pub fn with_settobello(mut self, id: &TeamId) -> Self {
        self.settobello = Option::Some(*id);
        self
    }

    pub fn with_premiere(mut self, id: &TeamId) -> Self {
        self.premiere = Option::Some(*id);
        self
    }

    pub fn points(&self, id: &TeamId) -> Points {
        let as_points = |maybe_id: Option<TeamId>| maybe_id.map_or(Points::default(), |id0| (if id0 == *id { 1 } else { 0 }).into());
        let scopas = *self.scopas.get(id).unwrap_or(&Points::default());
        let highest_card_count = as_points(self.highest_card_count);
        let highest_coins_count = as_points(self.highest_coins_count);
        let settobello = as_points(self.settobello);
        let premiere = as_points(self.premiere);
        scopas + highest_card_count + highest_coins_count + settobello + premiere
    }
}

#[cfg(test)]
mod test {
    use crate::domain::team::Team;

    use super::*;

    #[test]
    fn round_will_contain_scopas() {
        let id1 = Team::new("name").id();
        let id2 = Team::new("name").id();
        let id3 = Team::new("name").id();
        let round = Round::default()
            .with_scopas(&id1, 1.into())
            .with_scopas(&id2, 2.into());
        assert_eq!(round.points(&id1), 1.into());
        assert_eq!(round.points(&id2), 2.into());
        assert_eq!(round.points(&id3), 0.into());
    }

    #[test]
    fn round_may_contain_no_points_winners() {
        let id1 = Team::new("name").id();
        let id2 = Team::new("name").id();
        let round = Round::default();
        assert_eq!(round.points(&id1), 0.into());
        assert_eq!(round.points(&id2), 0.into());
    }

    #[test]
    fn round_will_contain_card_count_winner() {
        let id1 = Team::new("name").id();
        let id2 = Team::new("name").id();
        let round = Round::default()
            .with_highest_card_count(&id1);
        assert_eq!(round.points(&id1), 1.into());
        assert_eq!(round.points(&id2), 0.into());
    }
    
    #[test]
    fn round_will_contain_coins_count_winner() {
        let id1 = Team::new("name").id();
        let id2 = Team::new("name").id();
        let round = Round::default()
            .with_highest_coins_count(&id1);
        assert_eq!(round.points(&id1), 1.into());
        assert_eq!(round.points(&id2), 0.into());
    }
    
    #[test]
    fn round_will_contain_settobello_winner() {
        let id1 = Team::new("name").id();
        let id2 = Team::new("name").id();
        let round = Round::default()
            .with_settobello(&id1);
        assert_eq!(round.points(&id1), 1.into());
        assert_eq!(round.points(&id2), 0.into());
    }
    
    #[test]
    fn round_will_contain_premiere_winner() {
        let id1 = Team::new("name").id();
        let id2 = Team::new("name").id();
        let round = Round::default()
            .with_premiere(&id1);
        assert_eq!(round.points(&id1), 1.into());
        assert_eq!(round.points(&id2), 0.into());
    }
}