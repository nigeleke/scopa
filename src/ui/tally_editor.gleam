import gleam/int
import gleam/list
import gleam/option.{type Option}
import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

import derived/team_score.{type TeamScore}
import domain/score.{type Score}
import domain/tally.{type Tally}
import domain/team/id.{type TeamId}
import domain/team/name
import domain/team/status
import ui/glow

pub fn view(
  round: Int,
  team_scores: List(TeamScore),
  draft_tally: Tally,
  on_draft_tally_changed: fn(Tally) -> msg,
) -> Element(msg) {
  let category_column =
    category_column(round, draft_tally, on_draft_tally_changed)

  let leading_score = team_score.leading_score(team_scores)

  let team_columns =
    team_scores
    |> list.flat_map(team_column(
      _,
      leading_score,
      draft_tally,
      on_draft_tally_changed,
    ))

  let cells =
    category_column
    |> list.append(team_columns)
    |> list.map(as_cell)

  h.div([a.class("tally-editor")], cells)
}

fn as_cell(child: Element(msg)) -> Element(msg) {
  h.div([a.class("tally-editor__cell")], list.wrap(child))
}

fn category_column(
  round: Int,
  draft_tally: Tally,
  on_draft_tally_changed: fn(Tally) -> msg,
) -> List(Element(msg)) {
  let assigned_cards = option.is_some(draft_tally.cards)
  let assigned_coins = option.is_some(draft_tally.coins)
  let assigned_settebello = option.is_some(draft_tally.settebello)
  let assigned_premiera = option.is_some(draft_tally.premiera)

  let on_scopa = on_draft_tally_changed(draft_tally)
  let on_icon = fn(set: fn(Tally, Option(TeamId)) -> Tally) {
    let tally = draft_tally |> set(option.None)
    on_draft_tally_changed(tally)
  }
  let on_settebello = on_draft_tally_changed(draft_tally)

  [
    round_cell(round),
    icon(
      "scopa",
      option.Some("Scopa"),
      checked: True,
      disabled: False,
      on_click: on_scopa,
    ),
    icon(
      "carte",
      option.Some("Cards count"),
      checked: !assigned_cards,
      disabled: False,
      on_click: on_icon(tally.set_cards),
    ),
    icon(
      "denari",
      option.Some("Coins count"),
      checked: !assigned_coins,
      disabled: False,
      on_click: on_icon(tally.set_cards),
    ),
    icon(
      "settebello",
      option.Some("Settebello"),
      checked: !assigned_settebello,
      disabled: assigned_settebello,
      on_click: on_settebello,
    ),
    icon(
      "premiera",
      option.Some("Premiera"),
      checked: !assigned_premiera,
      disabled: False,
      on_click: on_icon(tally.set_premiera),
    ),
  ]
}

fn round_cell(value: Int) -> Element(msg) {
  let value_string = int.to_string(value)
  let attributes = [
    a.class("tally-editor__round"),
    a.data("round", value_string),
  ]
  let children = [h.text("Round: " <> value_string)]

  case int.is_even(value) {
    True -> h.div(attributes, children)
    False -> h.span(attributes, children)
  }
}

fn team_column(
  team_score: TeamScore,
  leading_score: Score,
  draft_tally: Tally,
  on_draft_tally_changed: fn(Tally) -> msg,
) -> List(Element(msg)) {
  let team_id = team_score.team_id(team_score)
  let some_team_id = option.Some(team_id)

  let scopa_points = tally.scopa_points(draft_tally, team_id)
  let has_cards = tally.awarded_cards(draft_tally, some_team_id)
  let has_coins = tally.awarded_coins(draft_tally, some_team_id)
  let has_settebello = tally.awarded_settebello(draft_tally, some_team_id)
  let has_premiera = tally.awarded_premiera(draft_tally, some_team_id)

  let disabled = team_score.team_status(team_score) == status.Eliminated

  let on_scopa = fn(value: Int) {
    let tally = draft_tally |> tally.set_scopas(team_id, value)
    on_draft_tally_changed(tally)
  }

  let on_icon = fn(set: fn(Tally, Option(TeamId)) -> Tally) {
    let tally = draft_tally |> set(some_team_id)
    on_draft_tally_changed(tally)
  }

  [
    team_header_cell(team_score, leading_score),
    scopa(scopa_points, disabled:, on_click: on_scopa),
    icon(
      "carte",
      option.None,
      checked: has_cards,
      disabled:,
      on_click: on_icon(tally.set_cards),
    ),
    icon(
      "denari",
      option.None,
      checked: has_coins,
      disabled:,
      on_click: on_icon(tally.set_coins),
    ),
    icon(
      "settebello",
      option.None,
      checked: has_settebello,
      disabled:,
      on_click: on_icon(tally.set_settebello),
    ),
    icon(
      "premiera",
      option.None,
      checked: has_premiera,
      disabled:,
      on_click: on_icon(tally.set_premiera),
    ),
  ]
}

fn team_header_cell(
  team_score: TeamScore,
  leading_score: Score,
) -> Element(msg) {
  let name = team_score.team_name(team_score)
  let score = team_score.score(team_score)

  let heading = name.to_string(name) <> ": " <> score.to_string(score)
  case score == leading_score && score != score.zero {
    True -> glow.view([h.text(heading)])
    False -> h.text(heading)
  }
}

fn scopa(
  scopas: Int,
  disabled disabled: Bool,
  on_click on_click: fn(Int) -> msg,
) -> Element(msg) {
  h.span(
    [
      a.class("tally-editor__scopa"),
      case disabled {
        True -> a.class("disabled")
        False -> a.none()
      },
      // TODO:
      event.on_click(on_click(0)),
    ],
    [
      h.text(int.to_string(scopas)),
    ],
  )
}

fn icon(
  name: String,
  hint: Option(String),
  checked checked: Bool,
  disabled disabled: Bool,
  on_click on_click: msg,
) -> Element(msg) {
  let asset = "/image/punteggio_" <> name <> ".png"

  let base = [
    h.img([
      a.class("tally-editor__icon"),
      case checked {
        True -> a.class("checked")
        False -> a.none()
      },
      case disabled {
        True -> a.class("disabled")
        False -> a.none()
      },
      a.src(asset),
      event.on_click(on_click),
    ]),
  ]

  let hint = case hint {
    option.Some(hint) -> h.span([a.class("tally-editor__hint")], [h.text(hint)])
    option.None -> element.none()
  }

  element.fragment(list.append(base, [hint]))
}
