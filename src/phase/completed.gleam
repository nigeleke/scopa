import derived/team_score.{type TeamScore}
import domain/target.{type Target}
import domain/team.{type Team}

pub type Model {
  Model(team_scores: List(TeamScore), winner: Team, target: Target)
}

pub fn init(
  team_scores: List(TeamScore),
  winner: Team,
  target: Target,
) -> Model {
  Model(team_scores:, winner:, target:)
}
