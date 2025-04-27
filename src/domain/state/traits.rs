use crate::domain::{
    RoundNumber,
    history::History,
    target::Target,
    teams::{Team, TeamName, Teams},
};

pub trait HasTeams {
    fn teams(&self) -> &Teams;
}

pub trait HasTarget {
    fn target(&self) -> Target;
}

pub trait HasHistory: HasTeams {
    fn history(&self) -> &History;

    fn round_number(&self) -> RoundNumber {
        self.history().round_number()
    }
}

pub trait HasWinner: HasTeams {
    fn winner(&self) -> &Team;

    fn winner_name(&self) -> &TeamName {
        self.winner().name()
    }
}
