use serde::{Deserialize, Serialize};

use crate::application::Page;
use crate::domain::{Game, GameState, Points, Round, RoundNumber, Target, Team, TeamId, TeamName};
use crate::i18n::Language;

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    page: Page,
    game: Game,
    language: Option<Language>,
}

impl Model {
    pub fn page(&self) -> Page {
        self.page
    }

    pub fn show_help(&mut self) {
        self.page = Page::Help
    }

    pub fn show_game(&mut self) {
        self.page = Page::Game
    }

    pub fn state(&self) -> GameState {
        self.game.state()
    }

    pub fn target(&self) -> Target {
        self.game.target()
    }

    pub fn set_target(&mut self, target: Target) {
        self.game.set_target(target);
    }

    pub fn can_add_team(&self, name: &TeamName) -> bool {
        self.game.can_add_team(name)
    }

    pub fn add_team(&mut self, name: TeamName) {
        self.game.add_team(name);
    }

    pub fn remove_team(&mut self, id: TeamId) {
        self.game.remove_team(id);
    }

    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        self.game.teams()
    }

    pub fn can_start(&self) -> bool {
        self.game.can_start()
    }

    pub fn start(&mut self) {
        self.game.start();
    }

    pub fn play(&mut self) {
        self.game.play();
    }

    pub fn active_teams(&self) -> impl Iterator<Item = &Team> {
        self.game.active_teams()
    }

    pub fn leading_teams(&self) -> impl Iterator<Item = &Team> {
        self.game.leading_teams()
    }

    pub fn winner(&self) -> Option<&Team> {
        self.game.winner()
    }

    pub fn score(&self, team_id: TeamId) -> Points {
        self.game.points(team_id)
    }

    pub fn round_number(&self) -> RoundNumber {
        self.game.round_number()
    }

    pub fn add_round(&mut self, round: Round) {
        self.game.add_round(round);
    }

    pub fn remove_round(&mut self) {
        self.game.remove_round();
    }

    pub fn last_round(&self) -> Option<&Round> {
        self.game.last_round()
    }

    pub fn language(self) -> Option<Language> {
        self.language
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = Some(language);
    }

    pub fn can_reset(&self) -> bool {
        self.page == Page::Game && self.game.state() != GameState::Setup
    }

    pub fn reset(&mut self) {
        self.game = Game::default();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::convert::TryInto;

    use crate::domain::{Round, Target, TeamName};
    use crate::test_support::{RoundBuilder, TeamScore};

    fn team(name: &str) -> TeamName {
        TeamName::new(name)
    }

    fn setup_game(model: &mut Model, players: usize) -> Vec<TeamId> {
        (1..=players).for_each(|n| {
            model.add_team(TeamName::new(&format!("Team {n}")));
        });
        model.teams().map(|team| *team.id()).collect()
    }

    fn setup_started_game(model: &mut Model, players: usize) -> Vec<TeamId> {
        let ids = setup_game(model, players);
        model.start();
        ids
    }

    #[test]
    fn initial_page_is_setup_page() {
        let model = Model::default();
        assert_eq!(model.page(), Page::Game);
        assert_eq!(model.state(), GameState::Setup);
    }

    #[test]
    fn can_show_help() {
        let mut model = Model::default();
        model.show_help();
        assert_eq!(model.page(), Page::Help);
        assert_eq!(model.state(), GameState::Setup);
    }

    #[test]
    fn default_has_no_teams() {
        let model = Model::default();
        assert_eq!(model.teams().count(), 0);
    }

    #[test]
    fn can_add_team() {
        let mut model = Model::default();
        model.add_team(team("Red"));
        assert_eq!(model.teams().count(), 1);
    }

    #[test]
    fn can_remove_team() {
        let mut model = Model::default();
        model.add_team(team("Red"));

        let id = model.teams().next().unwrap().id();
        model.remove_team(*id);

        assert_eq!(model.teams().count(), 0);
    }

    #[test]
    fn cannot_exceed_six_teams() {
        let mut model = Model::default();
        (0..6).for_each(|i| model.add_team(team(&format!("Team {i}"))));
        assert!(!model.can_add_team(&team("Team 7")));
    }

    #[test]
    fn can_set_target() {
        let mut model = Model::default();
        let target = Target::from(21);
        model.set_target(target);
        assert_eq!(model.target(), target);
    }

    #[test]
    fn default_target_is_11() {
        let model = Model::default();
        assert_eq!(model.target(), Target::from(11));
    }

    #[test]
    #[should_panic(expected = "invalid start in state Setup")]
    fn cannot_start_with_too_few_teams() {
        let mut model = Model::default();
        model.add_team(team("Red"));
        model.start();
    }

    #[test]
    #[should_panic(expected = "invalid start in state Setup")]
    fn cannot_start_with_five_teams() {
        let mut model = Model::default();
        (0..5).for_each(|i| model.add_team(team(&format!("Team {i}"))));
        model.start();
    }

    #[test]
    #[should_panic(expected = "invalid start in state Setup")]
    fn cannot_start_with_invalid_target() {
        let mut model = Model::default();
        let _ = setup_game(&mut model, 2);
        model.set_target(Target::from(0));
        model.start();
    }

    #[test]
    fn can_start_with_valid_initial_conditions() {
        let mut model = Model::default();
        let _ = setup_game(&mut model, 2);
        model.start();
        assert_eq!(model.state(), GameState::Playing);
    }

    #[test]
    fn can_score_rounds() {
        let mut model = Model::default();
        let _ = setup_started_game(&mut model, 2);

        assert_eq!(model.round_number(), RoundNumber::from(1));
        model.add_round(Round::default());
        assert_eq!(model.round_number(), RoundNumber::from(2));
    }

    #[test]
    #[should_panic(expected = "cannot add round unless playing")]
    fn cannot_score_round_during_setup() {
        let mut model = Model::default();
        let _ = setup_game(&mut model, 2);
        model.add_round(Round::default());
    }

    #[test]
    #[should_panic(expected = "cannot add round unless playing")]
    fn cannot_score_round_if_finished() {
        let mut model = Model::default();
        let [id1, _] = setup_started_game(&mut model, 2).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(10).settebello())
            .build();
        model.add_round(round);

        model.add_round(Round::default());
    }

    #[test]
    fn round_totals_are_added_to_scores() {
        let mut model = Model::default();
        let [id1, id2] = setup_started_game(&mut model, 2).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(3).cards().coins())
            .with_team_score(id2, TeamScore::default().scopas(2).settebello().premiera())
            .build();

        model.add_round(round);
        assert_eq!(model.score(id1), Points::from(5));
        assert_eq!(model.score(id2), Points::from(4));
    }

    #[test]
    fn undo_restores_previous_scores() {
        let mut model = Model::default();
        let [id1, id2] = setup_started_game(&mut model, 2).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(3).cards().coins())
            .with_team_score(id2, TeamScore::default().scopas(2).settebello().premiera())
            .build();

        model.add_round(round);
        assert_ne!(model.score(id1), Points::zero());
        assert_ne!(model.score(id2), Points::zero());

        model.remove_round();
        assert_eq!(model.score(id1), Points::zero());
        assert_eq!(model.score(id2), Points::zero());
    }

    #[test]
    fn undo_restores_eliminations() {
        let mut model = Model::default();
        let [id1, id2, id3, id4] = setup_started_game(&mut model, 4).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(10).coins())
            .with_team_score(id2, TeamScore::default().scopas(1))
            .with_team_score(id3, TeamScore::default().scopas(10).settebello())
            .with_team_score(id4, TeamScore::default().scopas(10).premiera())
            .build();

        model.add_round(round);
        assert_eq!(
            model.active_teams().map(|t| *t.id()).collect::<Vec<_>>(),
            [id1, id3, id4]
        );

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().coins())
            .with_team_score(id3, TeamScore::default().settebello())
            .build();
        model.add_round(round);
        assert_eq!(
            model.active_teams().map(|t| *t.id()).collect::<Vec<_>>(),
            [id1, id3]
        );

        model.remove_round();
        assert_eq!(
            model.active_teams().map(|t| *t.id()).collect::<Vec<_>>(),
            [id1, id3, id4]
        );

        model.remove_round();
        assert_eq!(
            model.active_teams().map(|t| *t.id()).collect::<Vec<_>>(),
            [id1, id2, id3, id4]
        );
    }

    #[test]
    #[should_panic(expected = "cannot remove round")]
    fn cannot_undo_first_round() {
        let mut model = Model::default();
        let _ = setup_game(&mut model, 2);
        model.remove_round();
    }

    #[test]
    fn no_winner_before_target_reached() {
        let mut model = Model::default();
        let [id1, id2] = setup_started_game(&mut model, 2).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(3).cards().coins())
            .with_team_score(id2, TeamScore::default().scopas(2).settebello().premiera())
            .build();

        model.add_round(round);
        assert_eq!(model.winner(), None);
        assert_eq!(model.state(), GameState::Playing);
    }

    #[test]
    fn game_ends_with_single_leader() {
        let mut model = Model::default();
        let [id1, id2] = setup_started_game(&mut model, 2).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(3).cards().coins())
            .with_team_score(id2, TeamScore::default().scopas(9).settebello().premiera())
            .build();

        model.add_round(round);
        assert_eq!(model.winner().map(|t| *t.id()), Some(id2));
        assert_eq!(model.state(), GameState::Finished);
    }

    #[test]
    fn game_continues_until_tie_broken() {
        let mut model = Model::default();
        let [id1, id2] = setup_started_game(&mut model, 2).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(10).cards().coins())
            .with_team_score(id2, TeamScore::default().scopas(10).settebello().premiera())
            .build();

        model.add_round(round);
        assert_eq!(model.winner(), None);
        assert_eq!(model.state(), GameState::Playing);
    }

    #[test]
    fn trailing_teams_drop_out_when_target_reached() {
        let mut model = Model::default();
        let [id1, id2, id3] = setup_started_game(&mut model, 3).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(10).cards().coins())
            .with_team_score(id3, TeamScore::default().scopas(10).settebello().premiera())
            .build();

        model.add_round(round);
        assert_eq!(model.state(), GameState::Playing);

        let leading_teams = model.leading_teams().map(|t| *t.id()).collect::<Vec<_>>();
        assert_eq!(leading_teams.len(), 2);
        assert!(leading_teams.contains(&id1));
        assert!(!leading_teams.contains(&id2));
        assert!(leading_teams.contains(&id3));

        assert_eq!(
            model.teams().map(|t| t.is_eliminated()).collect::<Vec<_>>(),
            [false, true, false]
        );

        assert_eq!(
            model
                .teams()
                .filter_map(|t| (!t.is_eliminated()).then_some(*t.id()))
                .collect::<Vec<_>>(),
            [id1, id3]
        );
    }

    #[test]
    fn trailing_teams_do_not_drop_out_when_target_not_reached() {
        let mut model = Model::default();
        let [id1, id2, id3] = setup_started_game(&mut model, 3).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(1).cards().coins())
            .with_team_score(id3, TeamScore::default().scopas(1).settebello().premiera())
            .build();

        model.add_round(round);
        assert_eq!(model.state(), GameState::Playing);

        let leading_teams = model.leading_teams().map(|t| *t.id()).collect::<Vec<_>>();
        assert_eq!(leading_teams.len(), 2);
        assert!(leading_teams.contains(&id1));
        assert!(!leading_teams.contains(&id2));
        assert!(leading_teams.contains(&id3));

        assert_eq!(
            model.teams().map(|t| t.is_eliminated()).collect::<Vec<_>>(),
            [false, false, false]
        );

        assert_eq!(
            model
                .teams()
                .filter_map(|t| (!t.is_eliminated()).then_some(*t.id()))
                .collect::<Vec<_>>(),
            [id1, id2, id3]
        );
    }

    #[test]
    fn restart_same_teams_should_reinstate_teams() {
        let mut model = Model::default();
        let [id1, id2, id3] = setup_started_game(&mut model, 3).try_into().unwrap();

        let round = RoundBuilder::default()
            .with_team_score(id1, TeamScore::default().scopas(10).cards().coins())
            .build();

        model.add_round(round);
        assert_eq!(model.state(), GameState::Finished);

        model.start();
        assert_eq!(model.state(), GameState::Playing);

        assert_eq!(
            model.teams().map(|t| t.is_eliminated()).collect::<Vec<_>>(),
            [false, false, false]
        );

        assert_eq!(
            model
                .teams()
                .filter_map(|t| (!t.is_eliminated()).then_some(*t.id()))
                .collect::<Vec<_>>(),
            [id1, id2, id3]
        );
    }
}
