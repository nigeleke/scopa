use serde::{Deserialize, Serialize};

use crate::domain::prelude::*;
use crate::domain::{GameState, InternalGameState};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayingState {
    teams: Vec<Team>,
    rounds: Vec<Round>,
    target: Target,
}

impl InternalGameState for PlayingState {}

impl PlayingState {
    pub fn new(teams: &[Team], target: Target) -> Self {
        Self {
            teams: Vec::from(teams),
            rounds: Vec::default(),
            target,
        }
    }

    pub fn target(&self) -> Target {
        self.target
    }

    pub fn score_round(&self, round: &Round) -> Result<GameState> {
        let mut new_state = self.clone();
        new_state.rounds.push(round.clone());

        let team_points = self
            .teams
            .iter()
            .map(|team: &Team| {
                let id = team.id();
                let points = self.points(id) + round.points(id);
                (team, points)
            })
            .collect::<Vec<_>>();

        type TeamPoints<'a> = Vec<(&'a Team, Points)>;
        let (winners, losers): (TeamPoints, TeamPoints) = team_points
            .iter()
            .partition(|(_, points)| *points >= self.target);

        let mut winners = winners;
        winners.sort_by(|(_, p1), (_, p2)| p2.cmp(p1));

        let no_winners = || winners.is_empty();
        let single_winner = || winners.len() == 1;
        let definitive_winner = || winners[0].1 > winners[1].1;

        let game = if no_winners() {
            GameState::Playing(Game::from(new_state))
        } else if single_winner() || definitive_winner() {
            GameState::Finished(Game::new_finished_state(
                &self.teams,
                &new_state.rounds,
                self.target,
                winners[0].0.id(),
            ))
        } else {
            let loser_ids = losers.iter().map(|(t, _)| t.id()).collect::<Vec<_>>();
            new_state.teams.iter_mut().for_each(|t| {
                if loser_ids.contains(&t.id()) {
                    t.set_not_playing();
                }
            });
            GameState::Playing(Game::from(new_state))
        };

        Ok(game)
    }
}

impl Teams for PlayingState {
    fn teams(&self) -> &[Team] {
        &self.teams
    }
}

impl Rounds for PlayingState {
    fn rounds(&self) -> &[Round] {
        &self.rounds
    }
}
