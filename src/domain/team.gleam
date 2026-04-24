import domain/score.{type Score}
import domain/team_name.{type TeamName}
import domain/team_status.{type TeamStatus}

pub type Team {
  Team(name: TeamName, score: Score, status: TeamStatus)
}
