import gleam/list

import domain/team.{type Team}

pub opaque type Teams {
  Teams(List(Team))
}

pub fn new() -> Teams {
  Teams(list.new())
}

pub fn append(teams: Teams, team: Team) -> Teams {
  let Teams(teams) = teams
  Teams(teams |> list.append([team]))
}

pub fn map(teams: Teams, f: fn(Team) -> a) -> List(a) {
  let Teams(teams) = teams
  teams |> list.map(f)
}
