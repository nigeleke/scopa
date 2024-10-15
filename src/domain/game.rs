use super::round::*;
use super::state::*;
use super::team::*;

use crate::types::*;

#[derive(Clone, Debug)]
pub enum GameState {
    Starting(Game<StartingState>),
    Playing(Game<PlayingState>),
    Finished(Game<FinishedState>),
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Starting(Game::default())
    }
}

impl std::ops::Deref for GameState {
    type Target = dyn InternalGameState;

    fn deref(&self) -> &Self::Target {
        match self {
            GameState::Starting(game) => &game.state,
            GameState::Playing(game) => &game.state,
            GameState::Finished(game) => &game.state,
        }
    }
}

pub trait InternalGameState {}

#[derive(Clone, Debug, PartialEq)]
pub struct Game<T> {
    state: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Game<T>
where
    T: Clone
{
    pub fn state(&self) -> T {
        self.state.clone()
    }
}

impl<T> std::ops::Deref for Game<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T> std::ops::DerefMut for Game<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl Default for Game<StartingState> {
    fn default() -> Self {
        Self {
            state: StartingState::default(),
            _phantom: std::marker::PhantomData
        }
    }
}

impl Game<PlayingState> {
    pub fn new_playing_state(teams: &[Team], target: Target) -> Self {
        Self {
            state: PlayingState::new(teams, target),
            _phantom: std::marker::PhantomData
        }
    }
}

impl From<PlayingState> for Game<PlayingState> {
    fn from(value: PlayingState) -> Self {
        Self {
            state: value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl Game<FinishedState> {
    pub fn new_finished_state(teams: &[Team], rounds: &[Round], target: Target, winner: TeamId) -> Self {
        Self {
            state: FinishedState::new(teams, rounds, target, winner),
            _phantom: std::marker::PhantomData
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::result::Error;
    use super::*;

    #[test]
    fn a_new_game_will_have_no_teams() {
        let game = Game::default();
        assert_eq!(game.team_count(), 0.into());
    }

    #[test]
    fn a_new_game_default_target_will_be_16() {
        let game = Game::default();
        assert_eq!(game.target(), 16.into());
    }

    #[test]
    fn teams_can_be_added_before_a_game_starts() {
        let mut game = Game::default();
        let team = Team::new("name");
        let _ = game.add_team(team).unwrap();
        assert_eq!(game.team_count(), 1.into());
    }

    #[test]
    fn teams_can_be_removed_before_a_game_starts() {
        let mut game = Game::default();
        let team = Team::new("name");
        let id = team.id();
        let _ = game.add_team(team).unwrap();
        let result = game.remove_team(id);
        assert!(result.is_ok());
        assert_eq!(game.team_count(), 0.into());
    }

    #[test]
    fn teams_cannot_be_removed_if_they_havent_been_added() {
        let mut game = Game::default();
        let valid_team = Team::new("name");
        let _ = game.add_team(valid_team).unwrap();
        let invalid_team = Team::new("name");
        let result = game.remove_team(invalid_team.id());
        assert!(result.is_err());
        assert_eq!(game.team_count(), 1.into());
    }

    #[test]
    fn can_define_valid_new_target() {
        let mut game = Game::default();
        let target = 11.into();
        assert_ne!(game.target(), target);
        game.set_target(target);
        assert_eq!(game.target(), target);
    }

    #[test]
    fn starting_a_game_requires_valid_team_count() {
        (0..=8).into_iter().for_each(|i| {
            let mut game = Game::default();
            (0..i).into_iter().for_each(|_| {
                let team = Team::new("name");
                let _ = game.add_team(team).unwrap();
            });
            assert_eq!(game.team_count(), i.into());
            let result = game.start();
            if [2, 3, 4, 6].contains(&i) {
                assert!(result.is_ok())
            } else {
                assert_eq!(result.unwrap_err(), Error::InvalidNumberOfTeams(i.into()))
            }
        });
    }

    #[test]
    fn all_teams_start_with_zero_score() {
        let mut game = Game::default();

        let team1 = Team::new("name");
        let id1 = team1.id();

        let team2 = Team::new("name");
        let id2 = team2.id();

        let _ = game.add_team(team1).unwrap();
        let _ = game.add_team(team2).unwrap();

        let GameState::Playing(game) = game.start().unwrap() else { panic!("Unexpected state")};
        assert_eq!(game.points(id1), Points::from(0));
        assert_eq!(game.points(id2), Points::from(0));
    }

    #[test]
    fn each_round_of_play_will_be_scored() {
        let mut game = Game::default();

        let team1 = Team::new("name");
        let id1 = team1.id();

        let team2 = Team::new("name");
        let id2 = team2.id();

        let _ = game.add_team(team1).unwrap();
        let _ = game.add_team(team2).unwrap();

        let GameState::Playing(game) = game.start().unwrap() else { panic!("Unexpected state") };
        let score = Round::default()
            .with_scopas(id1, 2.into())
            .with_scopas(id2, 3.into());

        let GameState::Playing(game) = game.score_round(&score).unwrap() else { panic!("Unexpected state") };
        assert_eq!(game.points(id1), Points::from(2));
        assert_eq!(game.points(id2), Points::from(3));        
    }

    fn test_is_finished(target: usize, score1: usize, score2: usize, is_finished: bool) {
        let mut game = Game::default();
        game.set_target(target.into());

        let team1 = Team::new("name");
        let id1 = team1.id();

        let team2 = Team::new("name");
        let id2 = team2.id();

        let _ = game.add_team(team1).unwrap();
        let _ = game.add_team(team2).unwrap();

        let GameState::Playing(game) = game.start().unwrap() else { panic!("Unexpected state") };
        let score = Round::default()
            .with_scopas(id1, score1.into())
            .with_scopas(id2, score2.into());

        match game.score_round(&score).unwrap() {
            GameState::Starting(game) => panic!("Unexpected state {:?}", game),
            GameState::Playing(game) => {
                assert_eq!(game.points(id1), Points::from(score1));
                assert_eq!(game.points(id2), Points::from(score2));
                assert!(!is_finished);
            },
            GameState::Finished(game) => {
                assert_eq!(game.points(id1), Points::from(score1));
                assert_eq!(game.points(id2), Points::from(score2));
                assert!(is_finished);
            },
        }
    }

    #[test]
    fn game_will_remain_playing_if_score_not_reached_target() {
        test_is_finished(11, 2, 3, false);
    }

    #[test]
    fn game_will_be_finished_if_score_reaches_target_and_unique_winner() {
        test_is_finished(1, 2, 3, true);
    }

    #[test]
    fn game_will_remain_playing_if_target_reached_and_tied_by_multiple_teams() {
        test_is_finished(1, 2, 2, false);
    }

    #[test]
    fn target_reached_and_tied_by_multiple_teams_eliminates_lesser_teams() {
        let mut game = Game::default();
        game.set_target(3.into());

        let team1 = Team::new("name");
        let id1 = team1.id();

        let team2 = Team::new("name");
        let id2 = team2.id();

        let team3 = Team::new("name");
        let id3 = team3.id();

        let _ = game.add_team(team1).unwrap();
        let _ = game.add_team(team2).unwrap();
        let _ = game.add_team(team3).unwrap();

        let GameState::Playing(game) = game.start().unwrap() else { panic!("Unexpected state") };

        let score = Round::default()
            .with_scopas(id1, 2.into())
            .with_scopas(id2, 4.into())
            .with_scopas(id3, 4.into());
        let GameState::Playing(game) = game.score_round(&score).unwrap() else { panic!("Unexpected state") };

        let teams = game.teams();
        assert!(teams[0].is_not_playing());
        assert!(!teams[1].is_not_playing());
        assert!(!teams[2].is_not_playing());
    }

}