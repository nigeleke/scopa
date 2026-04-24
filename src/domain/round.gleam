import domain/score.{type Score}
import domain/team_name.{type TeamName}

pub type Round =
  List(#(TeamName, Score))
