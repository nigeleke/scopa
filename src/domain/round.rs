use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{points::Points, teams::TeamId};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Round {
    scopas: HashMap<TeamId, Points>,
    highest_card_count: Option<TeamId>,
    highest_coins_count: Option<TeamId>,
    settebello: Option<TeamId>,
    premiera: Option<TeamId>,
}

impl Round {
    pub fn with_scopas(mut self, id: TeamId, count: Points) -> Self {
        self.scopas.insert(id, count);
        self
    }

    pub const fn with_highest_card_count(mut self, id: Option<TeamId>) -> Self {
        self.highest_card_count = id;
        self
    }

    pub const fn with_highest_coins_count(mut self, id: Option<TeamId>) -> Self {
        self.highest_coins_count = id;
        self
    }

    pub const fn with_settebello(mut self, id: TeamId) -> Self {
        self.settebello = Option::Some(id);
        self
    }

    pub const fn with_premiera(mut self, id: Option<TeamId>) -> Self {
        self.premiera = id;
        self
    }

    pub fn team_ids(&self) -> impl Iterator<Item = TeamId> + '_ {
        self.scopas
            .keys()
            .copied()
            .chain(self.highest_card_count)
            .chain(self.highest_coins_count)
            .chain(self.settebello)
            .chain(self.premiera)
    }

    pub fn points(&self, id: TeamId) -> Points {
        let as_points = |maybe_id: Option<TeamId>| {
            maybe_id.map_or_else(Points::default, |id0| {
                (if id0 == id { 1 } else { 0 }).into()
            })
        };
        let scopas = self.scopas(id);
        let highest_card_count = as_points(self.highest_card_count);
        let highest_coins_count = as_points(self.highest_coins_count);
        let settobello = as_points(self.settebello);
        let premiera = as_points(self.premiera);
        scopas + highest_card_count + highest_coins_count + settobello + premiera
    }

    pub fn scopas(&self, id: TeamId) -> Points {
        self.scopas.get(&id).map_or_else(Points::default, |&p| p)
    }

    pub const fn card_count(&self) -> Option<TeamId> {
        self.highest_card_count
    }

    pub const fn coins_count(&self) -> Option<TeamId> {
        self.highest_coins_count
    }

    pub const fn settebello(&self) -> Option<TeamId> {
        self.settebello
    }

    pub const fn premiera(&self) -> Option<TeamId> {
        self.premiera
    }

    pub const fn is_well_defined(&self) -> bool {
        self.settebello.is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_unscored() {
        let round = Round::default();
        assert!(round.scopas.is_empty());
        assert!(round.highest_card_count.is_none());
        assert!(round.highest_coins_count.is_none());
        assert!(round.settebello.is_none());
        assert!(!round.is_well_defined());
        assert!(round.premiera.is_none());
    }

    #[test]
    fn round_will_contain_scopas() {
        let id1 = TeamId::default();
        let id2 = TeamId::default();
        let id3 = TeamId::default();
        let round = Round::default()
            .with_scopas(id1, Points::from(1))
            .with_scopas(id2, Points::from(2));

        assert_eq!(round.scopas(id1), Points::from(1));
        assert_eq!(round.scopas(id2), Points::from(2));
        assert_eq!(round.scopas(id3), Points::from(0));

        assert_eq!(round.points(id1), Points::from(1));
        assert_eq!(round.points(id2), Points::from(2));
        assert_eq!(round.points(id3), Points::from(0));
    }

    #[test]
    fn round_may_contain_no_points_winners() {
        let id1 = TeamId::default();
        let id2 = TeamId::default();
        let round = Round::default();
        assert_eq!(round.points(id1), Points::from(0));
        assert_eq!(round.points(id2), Points::from(0));
    }

    #[test]
    fn round_will_contain_card_count_winner() {
        let id1 = TeamId::default();
        let id2 = TeamId::default();
        let round = Round::default().with_highest_card_count(Some(id1));

        assert_eq!(round.card_count(), Some(id1));

        assert_eq!(round.points(id1), Points::from(1));
        assert_eq!(round.points(id2), Points::from(0));
    }

    #[test]
    fn round_will_contain_coins_count_winner() {
        let id1 = TeamId::default();
        let id2 = TeamId::default();
        let round = Round::default().with_highest_coins_count(Some(id1));

        assert_eq!(round.coins_count(), Some(id1));

        assert_eq!(round.points(id1), Points::from(1));
        assert_eq!(round.points(id2), Points::from(0));
    }

    #[test]
    fn round_will_contain_settebello_winner() {
        let id1 = TeamId::default();
        let id2 = TeamId::default();
        let round = Round::default().with_settebello(id1);

        assert_eq!(round.settebello(), Some(id1));
        assert!(round.is_well_defined());

        assert_eq!(round.points(id1), Points::from(1));
        assert_eq!(round.points(id2), Points::from(0));
    }

    #[test]
    fn round_will_contain_premiera_winner() {
        let id1 = TeamId::default();
        let id2 = TeamId::default();
        let round = Round::default().with_premiera(Some(id1));

        assert_eq!(round.premiera(), Some(id1));

        assert_eq!(round.points(id1), Points::from(1));
        assert_eq!(round.points(id2), Points::from(0));
    }
}
