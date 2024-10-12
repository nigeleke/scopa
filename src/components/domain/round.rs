use crate::components::ui::prelude::*;

use crate::components::prelude::*;
use crate::domain::prelude::*;
use crate::types::TeamId;

use dioxus::prelude::*;

#[component]
pub fn RoundEditor(
    teams: Vec<Team>,
    round: Round,
    onchange: EventHandler<Round>,
) -> Element {
    let round = use_context_provider(move || Signal::new(round.clone()));

    use_effect(move || {
        onchange.call(round());
    });

    let first_active_team_id = teams.iter()
        .find(|t| t.is_playing())
        .map(Team::id)
        .unwrap();

    rsx! {
        div {
            class: "round-editor-container",
            { scoring_row(teams.clone(), Empty, team_header) },
            { scoring_row(teams.clone(), ScopaIcon, scopa_score(first_active_team_id)) },
            { scoring_row(teams.clone(), radio_none_icon(PointsGroup::CardsCount), radio_team_icon(PointsGroup::CardsCount)) },
            { scoring_row(teams.clone(), radio_none_icon(PointsGroup::CoinsCount), radio_team_icon(PointsGroup::CoinsCount)) },
            { scoring_row(teams.clone(), radio_none_icon(PointsGroup::Settebello), radio_team_icon(PointsGroup::Settebello)) },
            { scoring_row(teams.clone(), radio_none_icon(PointsGroup::Premiera), radio_team_icon(PointsGroup::Premiera)) },
        }
    }
}

#[component]
fn Empty() -> Element {
    rsx! { " " }
}

fn scoring_row<F, G> (
    teams: Vec<Team>,
    default: F,
    element: G
) -> Element
where
    F: Fn() -> Element,
    G: Fn(Team) -> Element,
{
    let some_teams = teams.into_iter().map(Option::Some);
    let tie_or_teams = 
        std::iter::once(Option::None)
            .chain(some_teams);

    rsx! {
        div {
            class: "round-editor-row",
            for team in tie_or_teams {
                div {
                    class: "round-editor-column",
                    if let Some(team) = team {
                        {element(team)}
                    } else {
                        {default()} 
                    }
                }
            }
        }
    }
}

fn team_header(team: Team) -> Element { rsx! { TeamHeader { team: team } } }

fn scopa_score(first_active_id: TeamId) -> impl Fn(Team) -> Element { move |team: Team| {
    let this_team_id = team.id();
    rsx! { ScopaScore { team: team, autofocus: this_team_id == first_active_id } }
}}

fn radio_none_icon(group: PointsGroup) -> impl Fn() -> Element { move ||
    rsx! { RadioTeamIcon { group: group, team: None } } 
}

fn radio_team_icon(group: PointsGroup) -> impl Fn(Team) -> Element { move |team: Team|
    rsx! { RadioTeamIcon { group: group, team: Some(team) } }
}

#[component]
fn TeamHeader(                                                                                                                                            
    team: Team
) -> Element {
    let game = use_context::<Game<PlayingState>>();

    rsx! {
        Glow {
            TeamNameView { value: team.name() }
            ": "
            PointsView { value: game.points(team.id()) }
        }
    }
}

#[component]
fn ScopaIcon() -> Element {
    rsx! {
        Icon {
            src: "./images/broom.png",
            height: "80px",
        } 
    }
}

#[component]
fn ScopaScore(
    team: Team,
    autofocus: bool,
) -> Element {
    let mut round = use_context::<Signal<Round>>();

    let id = team.id();
    let is_not_playing = team.is_not_playing();

    let draft = use_signal(move || round.read().scopas(id));

    let update_draft = move |points| {
        let new_round = round().with_scopas(id, points);
        round.set(new_round);
    };
    
    rsx! {
        PointsEditor {
            value: draft(),
            onchange: update_draft,
            disabled: is_not_playing,
            autofocus: autofocus,
        }    
    }
}

#[component]
fn RadioTeamIcon(
    group: PointsGroup,
    team: Option<Team>,
) -> Element {
    let id = team.as_ref().map(|team| team.id());
    let is_not_playing = team.as_ref().map(|team| team.is_not_playing());

    let mut round = use_context::<Signal<Round>>();

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
            PointsGroup::Settebello => round().with_settobello(id.unwrap()),
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
            onchange: update_draft,
            hidden: true,
            r#type: "radio",
            name: group.to_string(),
            disabled: is_not_playing.unwrap_or(false),
            checked: draft() == id,
        }
        CardsIcon { cids: cids }
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
