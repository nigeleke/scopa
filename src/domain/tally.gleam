import gleam/dict.{type Dict}
import gleam/option.{type Option}

import domain/score.{type Score}
import domain/team/id.{type TeamId}

pub type Tally {
  Tally(
    scopas: Dict(TeamId, Int),
    cards: Option(TeamId),
    coins: Option(TeamId),
    settebello: Option(TeamId),
    premiera: Option(TeamId),
  )
}

pub fn new() -> Tally {
  Tally(
    scopas: dict.new(),
    cards: option.None,
    coins: option.None,
    settebello: option.None,
    premiera: option.None,
  )
}

pub fn score(tally: Tally, team_id: TeamId) -> Score {
  let total =
    scopa_points(tally, team_id)
    + awarded_points(tally.cards, team_id)
    + awarded_points(tally.coins, team_id)
    + awarded_points(tally.settebello, team_id)
    + awarded_points(tally.premiera, team_id)

  score.from_int(total)
}

pub fn scopa_points(tally: Tally, team_id: TeamId) -> Int {
  let result = tally.scopas |> dict.get(team_id)
  case result {
    Ok(x) -> x
    Error(_) -> 0
  }
}

pub fn awarded_cards(tally: Tally, team_id: Option(TeamId)) -> Bool {
  tally.cards == team_id
}

pub fn awarded_coins(tally: Tally, team_id: Option(TeamId)) -> Bool {
  tally.coins == team_id
}

pub fn awarded_settebello(tally: Tally, team_id: Option(TeamId)) -> Bool {
  tally.settebello == team_id
}

pub fn awarded_premiera(tally: Tally, team_id: Option(TeamId)) -> Bool {
  tally.premiera == team_id
}

fn awarded_points(opt: Option(TeamId), team_id: TeamId) -> Int {
  case opt {
    option.Some(t) if t == team_id -> 1
    _ -> 0
  }
}

pub fn set_scopas(tally: Tally, team_id: TeamId, points: Int) -> Tally {
  let scopas = tally.scopas |> dict.insert(team_id, points)
  Tally(..tally, scopas:)
}

pub fn set_cards(tally: Tally, team_id: Option(TeamId)) -> Tally {
  let cards = team_id
  Tally(..tally, cards:)
}

pub fn set_coins(tally: Tally, team_id: Option(TeamId)) -> Tally {
  let coins = team_id
  Tally(..tally, coins:)
}

pub fn set_settebello(tally: Tally, team_id: Option(TeamId)) -> Tally {
  let settebello = team_id
  Tally(..tally, settebello:)
}

pub fn set_premiera(tally: Tally, team_id: Option(TeamId)) -> Tally {
  let premiera = team_id
  Tally(..tally, premiera:)
}

pub fn is_scoreable(tally: Tally) -> Bool {
  option.is_some(tally.settebello)
}
