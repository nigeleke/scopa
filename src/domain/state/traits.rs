use crate::domain::{
    history::History,
    points::Points,
    teams::{TeamId, Teams},
};

pub trait HasTeams {
    fn teams(&self) -> &Teams;
}

pub trait HasTarget {
    fn target(&self) -> Points;
}

pub trait HasHistory {
    fn history(&self) -> &History;
}

pub trait HasWinner {
    fn winner(&self) -> TeamId;
}
