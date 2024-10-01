use super::{PointsView, PointsEditor};
use super::TargetView;
use super::TeamNameView;

use crate::domain::*;
use crate::types::*;

use dioxus::prelude::*;

#[component]
pub fn PlayingGame() -> Element {
    use_context_provider(|| Signal::new(Round::default()));

    rsx! {
        Target {}
        Rounds {}
        ScoreButton {}
    }
}

#[component]
pub fn Target() -> Element {
    let game = use_context::<Signal<Game>>();
    let target = game().target();
    let current_round = game().round();

    rsx! {
        div { "Current round: " {current_round.to_string()} }
        div { "Target: " TargetView { value: target} }
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

#[component]
pub fn Rounds() -> Element {
    let game = use_context::<Signal<Game>>();

    let teams = game.read().teams();

    let tied_plus_teams = std::iter::once(Option::None)
            .chain(teams.clone()
                .into_iter()
                .map(Option::Some))
            .collect::<Vec<_>>();

    let as_header = move |team: &Team| {
        rsx! {
            th { 
                TeamNameView { value: team.name() } 
            } 
        } 
    };
    
    let as_total = move |team: &Team| {
        rsx! {
            th {
                PointsView { value: game.read().points(team.id()) }
            }
        }
    };

    let as_scopa = move |team: &Team| {
        rsx! {
            td {
                ScopaScore { id: team.id(), is_active: team.is_active() }
            }
        }
    };

    let as_id_selector = move |group: PointsGroup| {
        move |team: &Option<Team>| {
            let id = team.as_ref().map(|team| team.id());
            let is_active = team.as_ref().map(|team| team.is_active()).unwrap_or(true);
            rsx! {
                td {
                    TeamSelect { group: group, id, is_active }
                } 
            }
        }
    }; 

    rsx! {
        table {
            tr { td { "" }             td { "- tied -" } { teams.iter().map(as_header) }  }
            tr { th { "Total:"  }      td { "" }         { teams.iter().map(as_total) } }
            tr { td { "Scopas:"  }     td { "" }         { teams.iter().map(as_scopa) } }
            tr { td { "Card count:" }  { tied_plus_teams.iter().map(as_id_selector(PointsGroup::CardsCount)) } }
            tr { td { "Coins count:" } { tied_plus_teams.iter().map(as_id_selector(PointsGroup::CoinsCount)) } }
            tr { td { "Settebello:" }  td { "" }         { teams.into_iter().map(|team| as_id_selector(PointsGroup::Settebello)(&Some(team))) } }
            tr { td { "Premiera:" }    { tied_plus_teams.iter().map(as_id_selector(PointsGroup::Premiera)) } }
        }
    }
}

#[component]
fn ScopaScore(id: TeamId, is_active: bool) -> Element {
    let mut round = use_context::<Signal<Round>>();

    let mut draft = use_signal(move || Points::default());

    use_effect(move || {
        draft.set(round.read().scopas(id));
    });

    let update_draft = move |points| {
        draft.set(points);
        round.set(round().with_scopas(id, points));
    };

    rsx! {
        PointsEditor {
            value: draft(),
            onchange: update_draft,
            autofocus: true,
        }
    }
}

#[component]
fn TeamSelect(
    group: PointsGroup,
    id: Option<TeamId>,
    is_active: bool
) -> Element {
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

    rsx! {
        input {
            onchange: update_draft,
            r#type: "radio",
            name: group.to_string(),
            disabled: !is_active,
            checked: draft() == id,
        }
    }
}

#[component]
fn ScoreButton() -> Element {
    let mut context = use_context::<Signal<Game>>();

    let mut round = use_context::<Signal<Round>>();

    let can_score = round.read().is_well_defined();

    let record_score = move |_| {
        context.set(context().score_round(&round.read()).unwrap());
        dioxus_logger::tracing::info!("ScoreButton::resetting_round {:?}", round.read());
        round.set(Round::default());
        dioxus_logger::tracing::info!("ScoreButton::resetted_round {:?}", round.read());
    };

    rsx! {
        button {
            disabled: !can_score,
            onclick: record_score,
            "Score"
        }
    }
}