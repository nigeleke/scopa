use thiserror::*;

use super::{
    history::History,
    points::Points,
    round::Round,
    state::*,
    teams::{Team, TeamId, Teams},
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GameError {
    #[error("cannot start current game")]
    CannotStart,

    #[error("invalid team {0}")]
    InvalidTeam(TeamId),

    #[error("requires settebello to be assigned")]
    RequiresSettebello,
}

type Result<T> = std::result::Result<T, GameError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game<T> {
    state: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Game<T> {
    const fn new(state: T) -> Self {
        Self {
            state,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: HasTeams> HasTeams for Game<T> {
    fn teams(&self) -> &Teams {
        self.state.teams()
    }
}

impl<T: HasTarget> HasTarget for Game<T> {
    fn target(&self) -> Points {
        self.state.target()
    }
}

impl<T: HasHistory> HasHistory for Game<T> {
    fn history(&self) -> &History {
        self.state.history()
    }
}

impl<T: HasWinner> HasWinner for Game<T> {
    fn winner(&self) -> TeamId {
        self.state.winner()
    }
}

impl<T: HasTeams> Game<T> {
    fn validate_team(&self, id: TeamId) -> Result<()> {
        self.teams()
            .iter()
            .any(|t| t.id() == id)
            .then_some(())
            .ok_or(GameError::InvalidTeam(id))
    }
}

impl<T: HasHistory + HasTeams> Game<T> {
    fn validate_round(&self, round: &Round) -> Result<()> {
        if !round.is_well_defined() {
            return Err(GameError::RequiresSettebello);
        }

        round.team_ids().try_for_each(|id| self.validate_team(id))?;

        Ok(())
    }

    fn points(&self, id: TeamId) -> Result<Points> {
        self.validate_team(id)?;
        Ok(self.history().points(id))
    }
}

impl Game<Starting> {
    pub fn add_team(&mut self, team: Team) {
        self.state.add_team(team);
    }

    pub fn remove_team(&mut self, team_id: TeamId) -> Result<()> {
        self.validate_team(team_id)?;
        self.state.remove_team(team_id);
        Ok(())
    }

    pub const fn set_target(&mut self, target: Points) {
        self.state.set_target(target);
    }

    pub fn start(self) -> Result<Game<Playing>> {
        let playing_state = self
            .state
            .can_start()
            .then_some({
                let (teams, target) = self.state.into_parts();
                Playing::new(teams, History::default(), target)
            })
            .ok_or(GameError::CannotStart)?;
        Ok(Game::<_>::new(playing_state))
    }
}

impl Default for Game<Starting> {
    fn default() -> Self {
        Self::from(Points::from(11))
    }
}

impl From<Points> for Game<Starting> {
    fn from(value: Points) -> Self {
        let starting_state = Starting::from(value);
        Self::new(starting_state)
    }
}

#[derive(Debug)]
enum ScoreRoundResult {
    Playing(Game<Playing>),
    Finished(Game<Finished>),
}

impl Game<Playing> {
    pub fn score_round(self, round: &Round) -> Result<ScoreRoundResult> {
        self.validate_round(round)?;

        let (mut teams, mut history, target) = self.state.into_parts();
        history.record(&teams, round);

        let team_points = teams.iter_mut().map(|team: &mut Team| {
            let id = team.id();
            let points = history.points(id) + round.points(id);
            (team, points)
        });

        type TeamPoints<'a> = (&'a mut Team, Points);
        let (mut winners, mut losers): (Vec<TeamPoints>, Vec<TeamPoints>) =
            team_points.partition(|(_, points)| *points >= target);

        winners.sort_by(|(_, p1), (_, p2)| p2.cmp(p1));

        let no_winners = winners.is_empty();
        let single_winner = winners.len() == 1;
        let definitive_winner = || winners[0].1 > winners[1].1;

        let result = if no_winners {
            let playing_state = Playing::new(teams, history, target);
            ScoreRoundResult::Playing(Self::new(playing_state))
        } else if single_winner || definitive_winner() {
            let winner = winners[0].0.id();
            let finished_state = Finished::new(teams, history, winner, target);
            ScoreRoundResult::Finished(Game::<_>::new(finished_state))
        } else {
            let winner_score = winners[0].1;
            losers.iter_mut().for_each(|t| t.0.set_inactive());
            winners
                .iter_mut()
                .filter(|t| t.1 < winner_score)
                .for_each(|t| t.0.set_inactive());
            let playing_state = Playing::new(teams, history, target);
            println!("non def 2 {:?}", playing_state);
            ScoreRoundResult::Playing(Self::new(playing_state))
        };

        Ok(result)
    }
}

#[coverage(off)]
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn default_game_will_have_no_teams() {
        let game = Game::default();
        assert_eq!(game.teams().len(), 0);
    }

    #[test]
    fn default_game_target_will_be_11() {
        let game = Game::default();
        assert_eq!(game.target(), Points::from(11));
    }

    #[test]
    fn teams_can_be_added_before_a_game_starts() {
        let mut game = Game::default();
        let team = Team::from("name");
        let _ = game.add_team(team);
        assert_eq!(game.teams().len(), 1);
    }

    #[test]
    fn teams_can_be_removed_before_a_game_starts() {
        let mut game = Game::default();
        let team = Team::from("name");
        let id = team.id();
        game.add_team(team);
        game.remove_team(id).expect("valid removal");
        assert_eq!(game.teams().len(), 0);
    }

    #[test]
    fn teams_cannot_be_removed_if_they_havent_been_added() {
        let mut game = Game::default();
        let valid_team = Team::from("name");
        game.add_team(valid_team);
        let invalid_team = Team::from("name");
        let error = game
            .remove_team(invalid_team.id())
            .expect_err("invalid removal");
        assert!(matches!(error, GameError::InvalidTeam(_)));
        assert_eq!(game.teams().len(), 1);
    }

    #[test]
    fn can_define_valid_new_target() {
        let mut game = Game::default();
        let target = Points::from(16);
        assert_ne!(game.target(), target);
        game.set_target(target);
        assert_eq!(game.target(), target);
    }

    #[test]
    fn starting_a_game_requires_valid_team_count() {
        (0..=8).into_iter().for_each(|i| {
            let mut game = Game::default();
            (0..i).into_iter().for_each(|_| {
                let team = Team::from("name");
                game.add_team(team);
            });
            assert_eq!(game.teams().len(), i);
            let result = game.start();
            if [2, 3, 4, 6].contains(&i) {
                assert!(result.is_ok())
            } else {
                assert_eq!(result.expect_err("invalid start"), GameError::CannotStart)
            }
        });
    }

    #[test]
    fn all_teams_start_with_zero_score() {
        let mut game = Game::default();

        let team1 = Team::from("name");
        let id1 = team1.id();

        let team2 = Team::from("name");
        let id2 = team2.id();

        game.add_team(team1);
        game.add_team(team2);

        let Ok(game) = game.start() else {
            panic!("unexpected state")
        };
        assert_eq!(game.points(id1).expect("valid points"), Points::from(0));
        assert_eq!(game.points(id2).expect("valid points"), Points::from(0));
    }

    #[test]
    fn each_round_of_play_will_be_scored() {
        let mut game = Game::default();

        let team1 = Team::from("name");
        let id1 = team1.id();

        let team2 = Team::from("name");
        let id2 = team2.id();

        game.add_team(team1);
        game.add_team(team2);

        let Ok(game) = game.start() else {
            panic!("unexpected state")
        };
        let round = Round::default()
            .with_scopas(id1, Points::from(2))
            .with_scopas(id2, Points::from(3))
            .with_settebello(id1);

        let Ok(ScoreRoundResult::Playing(game)) = game.score_round(&round) else {
            panic!("unexpected state")
        };
        assert_eq!(game.points(id1).expect("valid points"), Points::from(3));
        assert_eq!(game.points(id2).expect("valid points"), Points::from(3));
    }

    #[test]
    fn settebello_must_be_allocated_in_order_to_score() {
        let mut game = Game::default();

        let team1 = Team::from("name");
        let team2 = Team::from("name");

        game.add_team(team1);
        game.add_team(team2);

        let Ok(game) = game.start() else {
            panic!("unexpected state")
        };

        let round = Round::default();
        let error = game.score_round(&round).expect_err("invalid score round");
        assert_eq!(error, GameError::RequiresSettebello);
    }

    #[test]
    fn will_not_score_invalid_players() {
        let mut game = Game::default();

        let team1 = Team::from("name");
        let id1 = team1.id();

        let team2 = Team::from("name");

        game.add_team(team1);
        game.add_team(team2);

        let Ok(game) = game.start() else {
            panic!("unexpected state")
        };

        let invalid_team = Team::from("name");
        let round = Round::default()
            .with_scopas(invalid_team.id(), Points::from(2))
            .with_settebello(id1);
        let error = game.score_round(&round).expect_err("invalid score round");

        let GameError::InvalidTeam(id) = error else {
            panic!("invalid error")
        };

        assert_eq!(id, invalid_team.id());
    }

    #[test]
    fn will_not_provide_points_for_invalid_players() {
        let mut game = Game::default();

        let team1 = Team::from("name");
        let team2 = Team::from("name");

        game.add_team(team1);
        game.add_team(team2);

        let Ok(game) = game.start() else {
            panic!("unexpected state")
        };

        let invalid_team = Team::from("name");
        let error = game.points(invalid_team.id()).expect_err("invalid points");

        let GameError::InvalidTeam(id) = error else {
            panic!("invalid error")
        };

        assert_eq!(id, invalid_team.id());
    }

    fn test_is_finished(target: usize, score1: usize, score2: usize, is_finished: bool) {
        let mut game = Game::from(Points::from(target));

        let team1 = Team::from("name");
        let id1 = team1.id();

        let team2 = Team::from("name");
        let id2 = team2.id();

        game.add_team(team1);
        game.add_team(team2);

        let Ok(game) = game.start() else {
            panic!("unexpected state")
        };
        let round = Round::default()
            .with_scopas(id1, Points::from(score1 - 1))
            .with_scopas(id2, Points::from(score2))
            .with_settebello(id1);

        match game.score_round(&round).expect("valid score") {
            ScoreRoundResult::Playing(game) => {
                assert_eq!(
                    game.points(id1).expect("valid points"),
                    Points::from(score1)
                );
                assert_eq!(
                    game.points(id2).expect("valid points"),
                    Points::from(score2)
                );
                assert!(!is_finished);
                assert_eq!(game.target(), Points::from(target));
            }
            ScoreRoundResult::Finished(game) => {
                assert_eq!(
                    game.points(id1).expect("valid points"),
                    Points::from(score1)
                );
                assert_eq!(
                    game.points(id2).expect("valid points"),
                    Points::from(score2)
                );
                assert!(is_finished);
                assert_eq!(game.winner(), if score1 > score2 { id1 } else { id2 });
                assert_eq!(game.target(), Points::from(target));
            }
        }
    }

    #[test]
    fn game_will_remain_playing_if_score_not_reached_target() {
        test_is_finished(11, 2, 3, false);
    }

    #[test]
    fn game_will_be_finished_if_score_reaches_target_and_unique_winner_1() {
        test_is_finished(1, 2, 3, true);
    }

    #[test]
    fn game_will_be_finished_if_score_reaches_target_and_unique_winner_2() {
        test_is_finished(1, 3, 2, true);
    }

    #[test]
    fn game_will_remain_playing_if_target_reached_and_tied_by_multiple_teams() {
        test_is_finished(1, 2, 2, false);
    }

    #[test]
    fn target_reached_and_tied_by_multiple_teams_eliminates_lesser_teams() {
        let mut game = Game::from(Points::from(3));

        let team1 = Team::from("name");
        let id1 = team1.id();

        let team2 = Team::from("name");
        let id2 = team2.id();

        let team3 = Team::from("name");
        let id3 = team3.id();

        let team4 = Team::from("name");
        let id4 = team4.id();

        game.add_team(team1);
        game.add_team(team2);
        game.add_team(team3);
        game.add_team(team4);

        let Ok(game) = game.start() else {
            panic!("unexpected state")
        };

        let round = Round::default()
            .with_settebello(id1)
            .with_scopas(id2, Points::from(4))
            .with_scopas(id3, Points::from(4))
            .with_scopas(id4, Points::from(3));
        let ScoreRoundResult::Playing(game) = game.score_round(&round).expect("valid game") else {
            panic!("unexpected state")
        };

        let teams = game.teams();
        assert!(teams[0].is_not_playing());
        assert!(!teams[1].is_not_playing());
        assert!(!teams[2].is_not_playing());
        assert!(teams[3].is_not_playing());
    }
}
