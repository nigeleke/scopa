import gleam/dict.{type Dict}
import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}
import gleam/option.{type Option}

import domain/score.{type Score}
import domain/team/id.{type TeamId}

pub type Tally {
  Tally(
    scopas: Dict(TeamId, Score),
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

pub fn encode(tally: Tally) -> Json {
  json.object([
    #("scopas", encode_scopas(tally.scopas)),
    #("cards", option.map(tally.cards, id.encode) |> option_to_json),
    #("coins", option.map(tally.coins, id.encode) |> option_to_json),
    #("settebello", option.map(tally.settebello, id.encode) |> option_to_json),
    #("premiera", option.map(tally.premiera, id.encode) |> option_to_json),
  ])
}

fn encode_scopas(scopas: Dict(TeamId, Score)) -> Json {
  scopas
  |> dict.to_list
  |> json.array(fn(pair) {
    let #(team_id, score) = pair
    json.object([
      #("team_id", id.encode(team_id)),
      #("score", score.encode(score)),
    ])
  })
}

fn option_to_json(opt: Option(Json)) -> Json {
  case opt {
    option.Some(value) -> value
    option.None -> json.null()
  }
}

pub fn decode() -> Decoder(Tally) {
  use scopas <- decode.field("scopas", decode_scopas())
  use cards <- decode.field("cards", decode.optional(id.decode()))
  use coins <- decode.field("coins", decode.optional(id.decode()))
  use settebello <- decode.field("settebello", decode.optional(id.decode()))
  use premiera <- decode.field("premiera", decode.optional(id.decode()))

  decode.success(Tally(
    scopas: scopas,
    cards: cards,
    coins: coins,
    settebello: settebello,
    premiera: premiera,
  ))
}

fn decode_scopas() -> Decoder(Dict(TeamId, Score)) {
  decode.list(decode_scopa_pair())
  |> decode.map(dict.from_list)
}

fn decode_scopa_pair() -> Decoder(#(TeamId, Score)) {
  use team_id <- decode.field("team_id", id.decode())
  use score <- decode.field("score", score.decode())
  decode.success(#(team_id, score))
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

pub fn scopa_score(tally: Tally, team_id: TeamId) -> Score {
  let result = tally.scopas |> dict.get(team_id)
  case result {
    Ok(score) -> score
    Error(_) -> score.zero
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

fn scopa_points(tally: Tally, team_id: TeamId) -> Int {
  score.value(scopa_score(tally, team_id))
}

pub fn set_scopas(
  tally: Tally,
  for team_id: TeamId,
  value points: Score,
) -> Tally {
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
