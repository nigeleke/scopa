mod number;

pub use number::RoundNumber;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::points::Points;
use crate::domain::team::TeamId;

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct Round {
    scopas: HashMap<TeamId, Points>,
    card_count: Option<TeamId>,
    coins_count: Option<TeamId>,
    settebello: Option<TeamId>,
    premiera: Option<TeamId>,
}

impl Round {
    pub fn scopas(&self, id: TeamId) -> Points {
        self.scopas.get(&id).map_or_else(Points::zero, |&p| p)
    }

    pub fn set_scopas(&mut self, id: TeamId, count: Points) {
        self.scopas.insert(id, count);
    }

    pub const fn card_count(&self) -> Option<TeamId> {
        self.card_count
    }

    pub const fn set_card_count(&mut self, id: Option<TeamId>) {
        self.card_count = id;
    }

    pub const fn coins_count(&self) -> Option<TeamId> {
        self.coins_count
    }

    pub const fn set_coins_count(&mut self, id: Option<TeamId>) {
        self.coins_count = id;
    }

    pub const fn settebello(&self) -> Option<TeamId> {
        self.settebello
    }

    pub const fn set_settebello(&mut self, id: TeamId) {
        self.settebello = Option::Some(id);
    }

    pub const fn premiera(&self) -> Option<TeamId> {
        self.premiera
    }

    pub const fn set_premiera(&mut self, id: Option<TeamId>) {
        self.premiera = id;
    }

    pub fn points(&self, id: TeamId) -> Points {
        let as_points = |maybe_id: Option<TeamId>| maybe_id.is_some_and(|id0| id0 == id).into();
        [
            self.scopas(id),
            as_points(self.card_count),
            as_points(self.coins_count),
            as_points(self.settebello),
            as_points(self.premiera),
        ]
        .into_iter()
        .sum()
    }

    pub const fn is_well_defined(&self) -> bool {
        self.settebello.is_some()
    }
}
