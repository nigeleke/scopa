use super::{
    points::Points,
    round::Round,
    teams::{TeamId, Teams},
};

#[derive(Debug)]
struct Entry {
    round: Round,
    teams: Teams,
}

#[derive(Debug, Default)]
pub struct History(Vec<Entry>);

impl History {
    pub fn record(&mut self, teams: &Teams, round: &Round) {
        let entry = Entry {
            round: round.clone(),
            teams: teams.clone(),
        };
        self.0.push(entry);
    }

    pub const fn round_number(&self) -> usize {
        self.0.len() + 1
    }

    pub fn points(&self, id: TeamId) -> Points {
        self.0
            .iter()
            .fold(Points::default(), |acc, r| acc + r.round.points(id))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::domain::teams::Team;

    #[test]
    fn default_history_is_empty() {
        let history = History::default();
        assert!(history.0.is_empty());
    }

    #[test]
    fn will_add_round_scores_to_the_history() {
        let team1 = Team::from("Alpha");
        let team2 = Team::from("Beta");
        let round = Round::default()
            .with_scopas(team1.id(), Points::from(2))
            .with_settebello(team2.id());
        let teams = Teams::from([team1, team2]);

        let mut history = History::default();
        history.record(&teams, &round);
        assert_eq!(history.0.len(), 1);
    }

    #[test]
    fn will_return_the_current_round_number() {
        let team1 = Team::from("Alpha");
        let team2 = Team::from("Beta");
        let round = Round::default()
            .with_scopas(team1.id(), Points::from(2))
            .with_settebello(team2.id());
        let teams = Teams::from([team1, team2]);

        let mut history = History::default();
        assert_eq!(history.round_number(), 1);
        history.record(&teams, &round);
        assert_eq!(history.round_number(), 2);
        history.record(&teams, &round);
        assert_eq!(history.round_number(), 3);
    }

    #[test]
    fn will_return_current_points_for_a_team() {
        let team1 = Team::from("Alpha");
        let id1 = team1.id();

        let team2 = Team::from("Beta");
        let id2 = team2.id();

        let round = Round::default()
            .with_scopas(id1, Points::from(2))
            .with_settebello(id2);
        let teams = Teams::from([team1, team2]);

        let mut history = History::default();
        history.record(&teams, &round);
        assert_eq!(history.points(id1), Points::from(2));
        assert_eq!(history.points(id2), Points::from(1));
    }
}
