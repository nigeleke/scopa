use crate::domain::{
    history::History,
    target::Target,
    teams::{TeamId, Teams},
};

pub trait HasTeams {
    fn teams(&self) -> &Teams;
}

pub trait HasTarget {
    fn target(&self) -> Target;
}

pub trait HasHistory: HasTeams {
    fn history(&self) -> &History;
}

pub trait HasWinner: HasTeams {
    fn winner(&self) -> TeamId;
}
