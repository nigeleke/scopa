use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;

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
        let is_not_playing = team.is_not_playing();
        [
            rsx! { TeamHeader { name: name, points: points } },
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
            class: "round-editor-component",
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
fn ScoringColumn(
    children: Element
) -> Element {
    rsx! { 
        div {
            class: "round-editor-column",
            {children} 
        }
    }
}

#[component]
fn Empty() -> Element {
    rsx! {
        div {
            class: "round-editor-item",
            p { " " } 
        }
    }
}

#[component]
fn TeamHeader(                                                                                                                                            
    name: TeamName,
    points: Points,
) -> Element {
    rsx! {
        Glow {
            class: "round-editor-item",
            TeamNameView { value: name }
            ": "
            PointsView { value: points }
        }
    }
}

#[component]
fn ScopaIcon() -> Element {
    rsx! {
        div {
            class: "round-editor-item",
            Icon {
                src: "./images/broom.png",
                height: "80px",
            } 
        }
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

    info!("ScopaScore {:?} af {} dis {}", id, autofocus, disabled);

    use_effect(move || {
        draft.set(round.read().scopas(id))
    });

    let update_draft = move |points| {
        round.set(round().with_scopas(id, points));
    };
    
    rsx! {
        div {
            class: "round-editor-item",
            PointsEditor {
                value: draft(),
                onchange: update_draft,
                autofocus: autofocus,
                disabled: disabled,
            }    
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

    let cids = match group {
        PointsGroup::CardsCount => "AC2H3S",
        PointsGroup::CoinsCount => "AD2D3D",
        PointsGroup::Settebello => "7D",
        PointsGroup::Premiera => "7H7C7D7S",
    };

    rsx! {
        input {
            value: draft() == id,
            onchange: update_draft,
            // hidden: true,
            r#type: "radio",
            name: group.to_string(),
            checked: draft() == id,
            disabled: disabled,
        }
        CardsIcon {
            cids: cids,
            disabled: disabled,
            checked: draft() == id,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PointsGroup {
    CardsCount,
    CoinsCount,
    Settebello,
    Premiera,
}

impl std::fmt::Display for PointsGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                PointsGroup::CardsCount => "cards-count",
                PointsGroup::CoinsCount => "coins-count",
                PointsGroup::Settebello => "settebello",
                PointsGroup::Premiera => "premiera",
            }
        })
    }
}
