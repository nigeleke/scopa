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

    pub fn finish(&mut self) {
        self.game.finish();
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
