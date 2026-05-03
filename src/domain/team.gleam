import domain/score.{type Score}

import domain/team/id.{type TeamId}
import domain/team/name.{type TeamName}
import domain/team/status.{type TeamStatus}

pub type Team {
  Team(id: TeamId, name: TeamName, score: Score, status: TeamStatus)
}

pub fn new(id: TeamId, name: TeamName) -> Team {
  Team(id, name, score: score.zero, status: status.Active)
}

pub fn name(team: Team) -> TeamName {
  let Team(_, name, _, _) = team
  name
}
