import gleam/int
import gleam/list
import gleam/option.{type Option}
import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

import domain/score.{type Score}
import domain/tally.{type Tally}
import domain/team.{type Team}
import domain/team/id.{type TeamId}
import domain/team/name
import domain/team/status
import domain/teams.{type Teams}
import ui/action/edit_scopa_score
import ui/glow

pub fn view(
  teams: Teams,
  round: Int,
  draft_tally: Tally,
  on_draft_tally_changed: fn(Tally) -> msg,
  on_edit_scopa_score: fn(TeamId) -> msg,
) -> Element(msg) {
  let category_column =
    category_column(round, draft_tally, on_draft_tally_changed)

  let leading_score = teams.leading_score(teams)

  let team_columns =
    teams
    |> teams.value
    |> list.flat_map(team_column(
      _,
      leading_score,
      draft_tally,
      on_draft_tally_changed,
      on_edit_scopa_score,
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
    // Force the pulse animation...
    True -> h.div(attributes, children)
    False -> h.span(attributes, children)
  }
}

fn team_column(
  team: Team,
  leading_score: Score,
  draft_tally: Tally,
  on_draft_tally_changed: fn(Tally) -> msg,
  on_edit_scopa_score: fn(TeamId) -> msg,
) -> List(Element(msg)) {
  let some_team_id = option.Some(team.id)

  let scopa_score = tally.scopa_score(draft_tally, team.id)
  let has_cards = tally.awarded_cards(draft_tally, some_team_id)
  let has_coins = tally.awarded_coins(draft_tally, some_team_id)
  let has_settebello = tally.awarded_settebello(draft_tally, some_team_id)
  let has_premiera = tally.awarded_premiera(draft_tally, some_team_id)

  let disabled = team.status == status.Eliminated

  let on_icon = fn(set: fn(Tally, Option(TeamId)) -> Tally) {
    let tally = draft_tally |> set(some_team_id)
    on_draft_tally_changed(tally)
  }

  [
    team_header_cell(team, leading_score),
    scopa(scopa_score, disabled:, on_edit: on_edit_scopa_score(team.id)),
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

fn team_header_cell(team: Team, leading_score: Score) -> Element(msg) {
  let heading = name.to_string(team.name) <> ": " <> score.to_string(team.score)
  case team.score == leading_score && team.score != score.zero {
    True -> glow.view([h.text(heading)])
    False -> h.text(heading)
  }
}

fn scopa(
  score: Score,
  disabled disabled: Bool,
  on_edit on_edit: msg,
) -> Element(msg) {
  edit_scopa_score.action(score, disabled, on_edit)
}

fn icon(
  name: String,
  hint: Option(String),
  checked checked: Bool,
  disabled disabled: Bool,
  on_click on_click: msg,
) -> Element(msg) {
  let asset = "/image/punteggio_" <> name <> ".png"

  let checked_class = case checked {
    True -> a.class("checked")
    False -> a.none()
  }

  let disabled_class = case disabled {
    True -> a.class("disabled")
    False -> a.none()
  }

  let tabindex = case name {
    "scopa" -> a.none()
    _ -> a.tabindex(0)
  }

  let base = [
    h.label(
      [
        a.class("tally-editor__icon"),
        tabindex,
        checked_class,
        disabled_class,
        event.on_click(on_click),
        event.on_keypress(fn(_) { on_click }),
      ],
      [
        h.input([
          a.type_("radio"),
          a.disabled(disabled),
        ]),
        h.img([a.src(asset)]),
      ],
    ),
  ]

  let hint = case hint {
    option.Some(hint) -> h.span([a.class("tally-editor__hint")], [h.text(hint)])
    option.None -> element.none()
  }

  element.fragment(list.append(base, [hint]))
}
