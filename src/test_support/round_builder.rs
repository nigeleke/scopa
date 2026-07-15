use crate::domain::{Points, Round, TeamId};
use crate::test_support::team_score::TeamScore;

#[derive(Default)]
pub struct RoundBuilder {
    round: Round,
}

#[cfg(test)]
impl RoundBuilder {
    pub fn with_team_score(mut self, id: TeamId, score: TeamScore) -> Self {
        self.round.set_scopas(id, Points::from(score.scopas));
        score.cards.then(|| self.round.set_card_count(Some(id)));
        score.coins.then(|| self.round.set_coins_count(Some(id)));
        score.settebello.then(|| self.round.set_settebello(id));
        score.premiera.then(|| self.round.set_premiera(Some(id)));
        self
    }

    pub fn build(self) -> Round {
        self.round
    }
}
