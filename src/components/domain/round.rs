use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn RoundEditor(
    state: PlayingState,
    round: Signal<Round>,
) -> Element {
    let teams = Vec::from(state.teams());

    let first_active_team_id = teams.iter()
        .find(|t| t.is_playing())
        .map(Team::id)
        .unwrap();

    let leading_team_score = teams.iter()
        .map(|team| state.points(team.id()))
        .max()
        .unwrap_or_default();

    let none_column_components = [
        rsx! { Empty {} },
        rsx! { ScopaIcon {} },
        rsx! { RadioTeamIcon { group: PointsGroup::CardsCount, id: None, round: round } },
        rsx! { RadioTeamIcon { group: PointsGroup::CoinsCount, id: None, round: round } },
        rsx! { RadioTeamIcon { group: PointsGroup::Settebello, id: None, round: round } },
        rsx! { RadioTeamIcon { group: PointsGroup::Premiera, id: None, round: round } },
    ];

    let rows_count = none_column_components.len();

    let team_column_components = move |team: &Team| {
        let id = team.id();
        let name = team.name();
        let points = state.points(id);
        let is_leader = points == leading_team_score;
        let is_not_playing = team.is_not_playing();
        [
            rsx! { TeamHeader { name: name, points: points, is_leader: is_leader } },
            rsx! { ScopaScore { id: id, round: round, autofocus: id == first_active_team_id, disabled: is_not_playing } },
            rsx! { RadioTeamIcon { group: PointsGroup::CardsCount, id: Some(id), round: round, disabled: is_not_playing } },
            rsx! { RadioTeamIcon { group: PointsGroup::CoinsCount, id: Some(id), round: round, disabled: is_not_playing } },
            rsx! { RadioTeamIcon { group: PointsGroup::Settebello, id: Some(id), round: round, disabled: is_not_playing } },
            rsx! { RadioTeamIcon { group: PointsGroup::Premiera, id: Some(id), round: round, disabled: is_not_playing } },
        ]
    };

    let some_column_components = teams.iter().map(team_column_components).collect::<Vec<_>>();
    let columns_count = some_column_components.len();

    rsx! {
        div {
            class: "round-editor-container",
            for i in 0..rows_count {
                div {
                    class: "round-editor-row",
                    div {
                        class: "round-editor-column",
                        {none_column_components[i].clone()}
                    }
                    for j in 0..columns_count {
                        div {
                            class: "round-editor-column",
                            {some_column_components[j][i].clone()}
                        }
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
fn TeamHeader(                                                                                                                                            
    name: TeamName,
    points: Points,
    is_leader: bool,
) -> Element {
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
fn ScopaIcon() -> Element {
    rsx! {
        Icon { src: asset!("./assets/images/broom.png") } 
    }
}

#[component]
fn ScopaScore(
    id: TeamId,
    round: Signal<Round>,
    autofocus: bool,
    disabled: bool,
) -> Element {
    let mut draft = use_signal(Points::default);

    use_effect(move || {
        draft.set(round.read().scopas(id))
    });

    let update_draft = move |points| {
        round.set(round().with_scopas(id, points));
    };
    
    rsx! {
        PointsEditor {
            value: draft(),
            onchange: update_draft,
            autofocus: autofocus,
            disabled: disabled,
        }
    }
}

#[component]
fn RadioTeamIcon(
    group: PointsGroup,
    id: Option<TeamId>,
    round: Signal<Round>,
    #[props(default = false)]
    disabled: bool,
) -> Element {
    let mut draft = use_signal(move || None);

    use_effect(move || {
        let selection = match group {
            PointsGroup::CardsCount => round.read().card_count(),
            PointsGroup::CoinsCount => round.read().coins_count(),
            PointsGroup::Settebello => round.read().settebello(),
            PointsGroup::Premiera => round.read().premiere(),
        };

        draft.set(selection);
    });
    
    let update_draft = move |_| {
        let new_round = match group {
            PointsGroup::CardsCount => round().with_highest_card_count(id),
            PointsGroup::CoinsCount => round().with_highest_coins_count(id),
            PointsGroup::Settebello => id.map_or(round(), |id| round().with_settobello(id)),
            PointsGroup::Premiera => round().with_premiere(id),
        };

        round.set(new_round);
    };

    let is_checked = *draft.read() == id;
    let settebello_none_disabled = group == PointsGroup::Settebello && id.is_none() && draft.read().is_some();
    let is_disabled = disabled || settebello_none_disabled;

    rsx! {
        label {
            class: "radio-team-icon",
            input {
                r#type: "radio",
                name: group.to_string(),
                value: format!("{}-{}", group.to_string(), id.map_or("none".to_string(), |t| t.to_string())),
                onchange: update_draft,
                checked: is_checked,
                disabled: is_disabled,
            }
            CardsImage {
                group: group,
                disabled: is_disabled,
                checked: is_checked,
            }    
        }
    }
}
