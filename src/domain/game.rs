mod state;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::domain::points::Points;
use crate::domain::round::{Round, RoundNumber};
use crate::domain::target::Target;
use crate::domain::team::{Team, TeamId, TeamName};

pub use state::GameState;

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    state: GameState,
    target: Target,
    teams: Vec<Team>,
    rounds: Vec<Round>,
}

const MAX_TEAMS: usize = 6;

impl Game {
    pub fn target(&self) -> Target {
        self.target
    }

    pub fn set_target(&mut self, target: Target) {
        match &mut self.state {
            GameState::Setup => self.target = target,
            _ => panic!("invalid set_target in state {:?}", self.state),
        }
    }

    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        self.teams.iter()
    }

    pub fn active_teams(&self) -> impl Iterator<Item = &Team> {
        self.teams.iter().filter(|t| !t.is_eliminated())
    }

    pub fn leading_teams(&self) -> impl Iterator<Item = &Team> {
        self.active_teams()
            .fold(BTreeMap::<Points, Vec<&Team>>::new(), |mut acc, team| {
                let score = self.points(*team.id());
                acc.entry(score).or_default().push(team);
                acc
            })
            .into_iter()
            .next_back()
            .into_iter()
            .flat_map(|(_, teams)| teams)
    }

    pub fn winner(&self) -> Option<&Team> {
        let reached_target = |team: &Team| self.points(*team.id()).value() >= self.target().value();

        let mut leaders = self.leading_teams();

        match (leaders.next(), leaders.next()) {
            (Some(team), None) if reached_target(team) => Some(team),
            _ => None,
        }
    }

    pub fn can_add_team(&self, name: &TeamName) -> bool {
        match &self.state {
            GameState::Setup => !name.is_empty() && self.teams.len() != MAX_TEAMS,
            _ => panic!("invalid can_add_team in state {:?}", self.state),
        }
    }

    pub fn add_team(&mut self, name: TeamName) {
        match &mut self.state {
            GameState::Setup => {
                let team = Team::new(name);
                self.teams.push(team);
            }
            _ => panic!("invalid add_team in state {:?}", self.state),
        }
    }

    pub fn remove_team(&mut self, id: TeamId) {
        match &mut self.state {
            GameState::Setup => self.teams.retain_mut(|team| team.id() != &id),
            _ => panic!("invalid remove_team in state {:?}", self.state),
        }
    }

    pub fn can_start(&self) -> bool {
        match &self.state {
            GameState::Setup => self.is_valid_team_count() && self.is_valid_target(),
            _ => panic!("invalid can_start in state {:?}", self.state),
        }
    }

    fn is_valid_team_count(&self) -> bool {
        matches!(self.teams.len(), 2 | 3 | 4 | 6)
    }

    fn is_valid_target(&self) -> bool {
        self.target != Target::from(0)
    }

    pub fn start(&mut self) {
        match &self.state {
            GameState::Setup if self.can_start() => {
                self.rounds = Vec::new();
                self.reinstate_teams();
                self.state = GameState::Playing
            }
            GameState::Finished => {
                let mut game = Game {
                    teams: Vec::from_iter(self.teams().cloned()),
                    ..Default::default()
                };
                game.start();
                *self = game;
            }
            _ => panic!("invalid start in state {:?}", self.state),
        }
    }

    pub fn play(&mut self) {
        match &self.state {
            GameState::Finished => self.state = GameState::Playing,
            _ => panic!("invalid play in state {:?}", self.state),
        }
    }

    pub fn round_number(&self) -> RoundNumber {
        RoundNumber::from(self.rounds.len() + 1)
    }

    pub fn add_round(&mut self, round: Round) {
        match &self.state {
            GameState::Playing => {
                self.rounds.push(round);
                self.eliminate_trailing_teams();
                if self.winner().is_some() {
                    self.state = GameState::Finished;
                }
            }
            _ => panic!("cannot add round unless playing"),
        }
    }

    fn eliminate_trailing_teams(&mut self) {
        let leaders = self
            .leading_teams()
            .map(|team| *team.id())
            .collect::<std::collections::HashSet<_>>();

        if let Some(id) = leaders.iter().next()
            && self.points(*id).value() >= self.target.value()
        {
            self.teams.iter_mut().for_each(|team| {
                if !leaders.contains(team.id()) {
                    team.eliminate();
                }
            });
        }
    }

    fn reinstate_teams(&mut self) {
        self.teams.iter_mut().for_each(|t| t.reinstate());
        self.eliminate_trailing_teams();
    }

    pub fn remove_round(&mut self) {
        self.rounds.pop().expect("cannot remove round");
        self.reinstate_teams();
    }

    pub fn last_round(&self) -> Option<&Round> {
        self.rounds.last()
    }

    pub fn points(&self, team_id: TeamId) -> Points {
        self.rounds.iter().map(|r| r.points(team_id)).sum()
    }

    pub fn state(&self) -> GameState {
        self.state
    }
}
