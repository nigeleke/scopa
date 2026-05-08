import domain/target.{type Target}
import domain/team.{type Team}
import domain/teams.{type Teams}

pub type Model {
  Model(teams: Teams, winner: Team, target: Target)
}

pub fn init(teams: Teams, winner: Team, target: Target) -> Model {
  Model(teams:, winner:, target:)
}
