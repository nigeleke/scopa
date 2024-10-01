use super::result::*;
use super::round::*;
use super::team::*;

use crate::types::*;

#[derive(Clone, Debug, PartialEq)]
enum GameState {
    Starting(Vec<Team>),
    Playing(Vec<Team>, Vec<Round>, Points),
    Finished(Vec<Team>, Vec<Round>, Points, TeamId),
}

impl Default for GameState {
    fn default() -> Self {
        Self::Starting(Vec::default())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game(GameState);

impl Game {
    pub fn new() -> Self {
        Self(GameState::default())
    }

    pub fn is_starting(&self) -> bool {
        match self.0 {
            GameState::Starting(_) => true,
            _ => false,
        }
    }

    pub fn is_playing(&self) -> bool {
        match self.0 {
            GameState::Playing(_, _, _) => true,
            _ => false,
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.0 {
            GameState::Finished(_, _, _, _) => true,
            _ => false,
        }
    }

    fn team_count(&self) -> TeamCount {
        match &self.0 {
            GameState::Starting(teams) |
            GameState::Playing(teams, _, _) |
            GameState::Finished(teams, _, _, _)=> teams.len(),
        }.into()
    }

    pub fn target(&self) -> Points {
        match &self.0 {
            GameState::Playing(_, _, target) => *target,
            _ => Points::default(),
        }
    }

    pub fn round(&self) -> usize {
        match &self.0 {
            GameState::Starting(_) => 0,
            GameState::Playing(_, rounds, _) |
            GameState::Finished(_, rounds, _, _) => rounds.len() + 1
        }
    }

    pub fn add_team(&mut self, name: &str) -> Result<TeamId> {
        match &mut self.0 {
            GameState::Starting(teams) => {
                let team = Team::new(name);
                let id = team.id();
                teams.push(team);
                Ok(id)
            },
            _ => Err(Error::TeamsCannotBeChanged)
        }
    }

    pub fn remove_team(&mut self, id: &TeamId) -> Result<()> {
        match &mut self.0 {
            GameState::Starting(teams) => {
                let valid = teams.iter().filter_map(|t| Some(t.id())).collect::<Vec<_>>().contains(id);
                teams.retain(|t| t.id() != *id);
                valid.then_some(()).ok_or(Error::TeamsCannotBeChanged)
            },
            _ => Err(Error::TeamsCannotBeChanged)
        }
    }

    const VALID_TEAM_COUNTS: [TeamCount; 4] = [TeamCount::new(2), TeamCount::new(3), TeamCount::new(4), TeamCount::new(6)];

    pub fn can_start(&self) -> bool {
        Self::VALID_TEAM_COUNTS.contains(&self.team_count())
    }

    pub fn start(self, target: Points) -> Result<Game> {
        match self.0 {
            GameState::Starting(teams) => {
                let team_count = teams.len().into();
                if Game::VALID_TEAM_COUNTS.contains(&team_count) {
                    Ok(Game(GameState::Playing(teams, Vec::default(), target)))
                } else {
                    Err(Error::InvalidNumberOfTeams)
                }
            },
            _ => Ok(self),
        }
    }

    pub fn points(&self, id: TeamId) -> Points {
        match &self.0 {
            GameState::Starting(_) => Points::default(),
            GameState::Playing(_, rounds, _) | 
            GameState::Finished(_, rounds, _, _) => {
                rounds.iter().fold(Points::default(), |acc, r| acc + r.points(id))
            },
        }
    }

    pub fn score_round(self, score: &Round) -> Result<Game> {        
        match self.0 {
            GameState::Starting(_) => Err(Error::GameNotStarted),
            GameState::Playing(ref teams, ref rounds, target) => {
                // let mut teams = teams.clone();

                let mut rounds = rounds.clone();
                rounds.push(score.clone());

                let team_points = teams
                    .iter()
                    .map(|team: &Team| {
                        let id = team.id();
                        let points = self.points(id) + score.points(id);
                        (team, points)
                    })
                    .collect::<Vec<_>>();

                let (winners, losers): (Vec<(&Team, Points)>, Vec<(&Team, Points)>) = team_points
                    .iter()
                    .partition(|(_, points)| *points >= target);

                let mut winners = winners;
                winners.sort_by(|(_, p1), (_, p2)| p2.cmp(p1));

                let game = if winners.is_empty() {
                    Game(GameState::Playing(teams.clone(), rounds.clone(), target))
                } else if winners.len() == 1 {
                    Game(GameState::Finished(teams.clone(), rounds.clone(), target, winners[0].0.id()))
                } else {
                    if winners[0].1 > winners[1].1 {
                        Game(GameState::Finished(teams.clone(), rounds.clone(), target, winners[0].0.id()))
                    } else {
                        let mut teams = losers.into_iter().map(|(team, _)| {
                            let mut team = team.clone();
                            team.disengage();
                            team
                        }).collect::<Vec<_>>();
                        let mut winners = winners.into_iter().map(|(team, _)| {
                            team.clone()
                        }).collect::<Vec<_>>();
                        teams.append(&mut winners);
                        Game(GameState::Playing(teams, rounds.clone(), target))
                    }
                };

                Ok(game)
            },            
            GameState::Finished(_, _, _, _) => Err(Error::GameAlreadyComplete),
        }
    }

    pub fn teams(&self) -> Vec<Team> {
        match &self.0 {
            GameState::Starting(teams) => teams,
            GameState::Playing(teams, _, _) => teams,
            GameState::Finished(teams, _, _, _) => teams,
        }.clone()
    }

    fn active_teams(&self) -> Vec<TeamId> {
        let teams = match &self.0 {
            GameState::Starting(teams) => teams,
            GameState::Playing(teams, _, _) => teams,
            GameState::Finished(teams, _, _, _) => teams,
        };
        teams.iter().filter_map(|team| team.is_active().then_some(team.id())).collect::<Vec<_>>()
    }

    fn inactive_teams(&self) -> Vec<TeamId> {
        let teams = match &self.0 {
            GameState::Starting(teams) => teams,
            GameState::Playing(teams, _, _) => teams,
            GameState::Finished(teams, _, _, _) => teams,
        };
        teams.iter().filter_map(|team| (!team.is_active()).then_some(team.id())).collect::<Vec<_>>()
    }

    pub fn winner(&self) -> Option<TeamName> {
        match &self.0 {
            GameState::Starting(_) |
            GameState::Playing(_, _, _) => None,
            GameState::Finished(teams, _, _, winner) => {
                teams
                    .iter()
                    .find(|t| t.id() == *winner)
                    .map(Team::name)
            },
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn an_initial_game_will_have_no_teams() {
        let game = Game::new();
        assert_eq!(game.team_count(), 0.into());
    }

    #[test]
    fn teams_can_be_added_before_a_game_starts() {
        let mut game = Game::new();
        let _ = game.add_team("name").unwrap();
        assert_eq!(game.team_count(), 1.into());
    }

    #[test]
    fn teams_can_be_removed_before_a_game_starts() {
        let mut game = Game::new();
        let id = game.add_team("name").unwrap();
        let result = game.remove_team(&id);
        assert!(result.is_ok());
        assert_eq!(game.team_count(), 0.into());
    }

    #[test]
    fn teams_cannot_be_removed_if_they_havent_been_added() {
        let mut game = Game::new();
        let _ = game.add_team("name").unwrap();
        let team = Team::new("name");
        let result = game.remove_team(&team.id());
        assert!(result.is_err());
        assert_eq!(game.team_count(), 1.into());
    }

    #[test]
    fn starting_a_game_requires_a_target_score() {
        let mut game = Game::new();
        let _ = game.add_team("name").unwrap();
        let _ = game.add_team("name").unwrap();
        let game = game.start(11.into()).unwrap();
        assert_eq!(game.target(), 11.into());
    }

    #[test]
    fn starting_a_game_requires_valid_team_count() {
        (0..=8).into_iter().for_each(|i| {
            let mut game = Game::new();
            (0..i).into_iter().for_each(|_| {
                game.add_team("name").unwrap();
            });
            assert_eq!(game.team_count(), i.into());
            let result = game.start(11.into());
            if [2, 3, 4, 6].contains(&i) {
                assert!(result.is_ok())
            } else {
                assert_eq!(result, Err(Error::InvalidNumberOfTeams))
            }
        });
    }

    #[test]
    fn all_teams_start_with_zero_score() {
        let mut game = Game::new();
        let id1 = game.add_team("name").unwrap();
        let id2 = game.add_team("name").unwrap();
        let game = game.start(11.into()).unwrap();
        assert_eq!(game.points(id1), 0.into());
        assert_eq!(game.points(id2), 0.into());
    }

    #[test]
    fn teams_cannot_be_added_after_a_game_has_started() {
        let mut game = Game::new();
        let _ = game.add_team("name").unwrap();
        let _ = game.add_team("name").unwrap();
        let mut game = game.start(11.into()).unwrap();
        let error = game.add_team("name").unwrap_err();
        assert_eq!(error, Error::TeamsCannotBeChanged);
    }

    #[test]
    fn teams_cannot_be_removed_after_a_game_has_started() {
        let mut game = Game::new();
        let _ = game.add_team("name").unwrap();
        let id = game.add_team("name").unwrap();
        let mut game = game.start(11.into()).unwrap();
        let error = game.remove_team(&id).unwrap_err();
        assert_eq!(error, Error::TeamsCannotBeChanged);
    }

    #[test]
    fn each_round_of_play_will_be_scored() {
        let mut game = Game::new();
        let id1 = game.add_team("name").unwrap();
        let id2 = game.add_team("name").unwrap();
        let game = game.start(11.into()).unwrap();
        let score = Round::default()
            .with_scopas(id1, 2.into())
            .with_scopas(id2, 3.into());
        let game = game.score_round(&score).unwrap();
        assert_eq!(game.points(id1), 2.into());
        assert_eq!(game.points(id2), 3.into());
    }

    fn test_is_finished(target: usize, score1: usize, score2: usize, is_finished: bool) {
        let mut game = Game::new();
        let id1 = game.add_team("name").unwrap();
        let id2 = game.add_team("name").unwrap();
        let game = game.start(target.into()).unwrap();
        let score = Round::default()
            .with_scopas(id1, score1.into())
            .with_scopas(id2, score2.into());
        let game = game.score_round(&score).unwrap();
        assert_eq!(game.points(id1), score1.into());
        assert_eq!(game.points(id2), score2.into());
        assert_eq!(game.is_finished(), is_finished);

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
        let mut game = Game::new();
        let id1 = game.add_team("name").unwrap();
        let id2 = game.add_team("name").unwrap();
        let id3 = game.add_team("name").unwrap();
        let game = game.start(3.into()).unwrap();
        let score = Round::default()
            .with_scopas(id1, 2.into())
            .with_scopas(id2, 4.into())
            .with_scopas(id3, 4.into());
        let game = game.score_round(&score).unwrap();
        assert_eq!(game.teams().iter().map(Team::id).collect::<Vec<_>>(), [id1, id2, id3]);
        assert_eq!(game.active_teams(), [id2, id3]);
        assert_eq!(game.inactive_teams(), [id1]);
    }

    #[test]
    fn rounds_cannot_be_scored_if_the_game_has_not_started() {
        let mut game = Game::new();
        let _ = game.add_team("name").unwrap();
        let _ = game.add_team("name").unwrap();
        let score = Round::default();
        let error = game.score_round(&score).unwrap_err();
        assert_eq!(error, Error::GameNotStarted);
    }

    #[test]
    fn rounds_cannot_be_scored_if_the_game_has_finished() {
        let mut game = Game::new();
        let id = game.add_team("name").unwrap();
        let _ = game.add_team("name").unwrap();
        let game = game.start(1.into()).unwrap();
        let score = Round::default()
            .with_scopas(id, 2.into());
        let game = game.score_round(&score).unwrap();
        let error = game.score_round(&score).unwrap_err();
        assert_eq!(error, Error::GameAlreadyComplete);
    }

}