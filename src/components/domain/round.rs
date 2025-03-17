use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_i18n::tid;

#[component]
pub fn RoundEditor(state: PlayingState, round: Signal<Round>) -> Element {
    let teams = Vec::from(state.teams());

    let first_active_team = teams.iter().find(|t| t.is_playing()).unwrap();
    let first_active_team_id = first_active_team.id();

    let leading_team_score = teams
        .iter()
        .map(|team| state.points(team.id()))
        .max()
        .unwrap_or_default();

    let none_column_components = [
        rsx! { Empty {} },
        rsx! { ScopaIcon { hint: tid!("scopa-icon.hint") } },
        rsx! { RadioTeamIcon { hint: tid!("cards-count-icon.hint"), group: PointsGroup::CardsCount, team: None, round: round } },
        rsx! { RadioTeamIcon { hint: tid!("coins-count-icon.hint"), group: PointsGroup::CoinsCount, team: None, round: round } },
        rsx! { RadioTeamIcon { hint: tid!("settebello-icon.hint"), group: PointsGroup::Settebello, team: None, round: round } },
        rsx! { RadioTeamIcon { hint: tid!("premiera-icon.hint"), group: PointsGroup::Premiera, team: None, round: round } },
    ];

    let team_column_components = move |team: &Team| {
        let id = team.id();
        let name = team.name();
        let points = state.points(id);
        let is_leader = points == leading_team_score;
        let is_not_playing = team.is_not_playing();
        [
            rsx! { TeamHeader { name: name, points: points, is_leader: is_leader } },
            rsx! { ScopaScore { team: team.clone(), round: round, autofocus: id == first_active_team_id, disabled: is_not_playing } },
            rsx! { RadioTeamIcon { group: PointsGroup::CardsCount, team: Some(team.clone()), round: round, disabled: is_not_playing } },
            rsx! { RadioTeamIcon { group: PointsGroup::CoinsCount, team: Some(team.clone()), round: round, disabled: is_not_playing } },
            rsx! { RadioTeamIcon { group: PointsGroup::Settebello, team: Some(team.clone()), round: round, disabled: is_not_playing } },
            rsx! { RadioTeamIcon { group: PointsGroup::Premiera, team: Some(team.clone()), round: round, disabled: is_not_playing } },
        ]
    };

    let some_column_components = teams.iter().map(team_column_components).collect::<Vec<_>>();
    let columns_count = some_column_components.len() + 1;

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/round.css") }
        div {
            class: "round-editor-container",
            for j in 0..columns_count {
                for i in 0..6 {
                    if j == 0 {
                        {none_column_components[i].clone()}
                    } else {
                        {some_column_components[j-1][i].clone()}
                    }
                }
            }
        }
    }
}

#[component]
fn Empty() -> Element {
    rsx! {
        div {
            p { " " }
        }
    }
}

#[component]
fn TeamHeader(name: TeamName, points: Points, is_leader: bool) -> Element {
    rsx! {
        if is_leader {
            span {
                class: "team-leader-header",
                Glow {
                    TeamNameView { value: name }
                    ": "
                    PointsView { value: points }
                }
            }

        } else {
            span {
                class: "team-header",
                TeamNameView { value: name }
                ": "
                PointsView { value: points }
            }

        }
    }
}

#[component]
fn ScopaIcon(hint: Option<String>) -> Element {
    rsx! {
        PointsGroupImage { hint, group: PointsGroup::Scopa, disabled: false, checked: true }
    }
}

#[component]
fn ScopaScore(team: Team, round: Signal<Round>, autofocus: bool, disabled: bool) -> Element {
    let id = team.id();
    let name = &team.name();

    let mut draft = use_signal(Points::default);

    use_effect(move || draft.set(round.read().scopas(id)));

    let update_draft = move |points| {
        round.set(round().with_scopas(id, points));
    };

    rsx! {
        PointsEditor {
            value: draft(),
            onchange: update_draft,
            autofocus: autofocus,
            disabled: disabled,
            aria_label: tid!("score-scopa-editor.aria-label", teamname: name.to_string()),
        }
    }
}

#[component]
fn RadioTeamIcon(
    hint: Option<String>,
    group: PointsGroup,
    team: Option<Team>,
    round: Signal<Round>,
    #[props(default = false)] disabled: bool,
) -> Element {
    let id = team.clone().map(|t| t.id());
    let name = team.map(|t| t.name());

    let mut draft = use_signal(move || None);

    use_effect(move || {
        let selection = match group {
            PointsGroup::Scopa => unreachable!(),
            PointsGroup::CardsCount => round.read().card_count(),
            PointsGroup::CoinsCount => round.read().coins_count(),
            PointsGroup::Settebello => round.read().settebello(),
            PointsGroup::Premiera => round.read().premiera(),
        };

        draft.set(selection);
    });

    let update_draft = move |_| {
        let new_round = match group {
            PointsGroup::Scopa => unreachable!(),
            PointsGroup::CardsCount => round().with_highest_card_count(id),
            PointsGroup::CoinsCount => round().with_highest_coins_count(id),
            PointsGroup::Settebello => id.map_or(round(), |id| round().with_settobello(id)),
            PointsGroup::Premiera => round().with_premiera(id),
        };

        round.set(new_round);
    };

    let is_checked = *draft.read() == id;
    let settebello_none_disabled =
        group == PointsGroup::Settebello && id.is_none() && draft.read().is_some();
    let is_disabled = disabled || settebello_none_disabled;

    rsx! {
        label {
            class: "radio-team-icon",
            Input {
                typ: "radio",
                name: group.to_string(),
                value: format!("{}-{}", group.to_string(), id.map_or("none".to_string(), |t| t.to_string())),
                on_input: update_draft,
                aria_label: tid!("score-group-icon.aria-label", group: group.to_string(), teamname: name.map_or("no one".to_string(), |n| n.to_string())),
                checked: is_checked,
                disabled: is_disabled,
            }
            PointsGroupImage {
                hint,
                group: group,
                disabled: is_disabled,
                checked: is_checked,
            }
        }
    }
}
