use crate::components::prelude::*;
use crate::domain::*;
use crate::types::*;

use dioxus::prelude::*;
use dioxus_logger::tracing::debug;
use dioxus_logger::tracing::info;

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
    use_effect(move || {
        debug!("game changed: {:?}", game);
    });

    let teams = game.read().teams();
    debug!("teams now: {:?}", teams);

    let some_teams = teams.iter().map(Option::Some);
    let tie_or_teams = 
        std::iter::once(Option::None)
            .chain(some_teams)
            .collect::<Vec<_>>();

    let as_header = move |team: &Team| {
        rsx! {
            th { 
                TeamNameView { value: team.name() } 
            } 
        } 
    };
    let headers = teams.iter().map(as_header);

    let as_total = move |team: &Team| {
        rsx! {
            th {
                PointsView { value: game.read().points(team.id()) }
            }
        }
    };
    let totals = teams.iter().map(as_total);

    let as_scopa = move |(i, team): (usize, &Team)| {
        info!("as_scopa team: {:?}", team);
        rsx! {
            td {
                ScopaScore { team: team.clone(), autofocus: i == 0 }
            }
        }
    };
    let scopas = teams.iter().enumerate().map(as_scopa);

    let as_id_selector = move |group: PointsGroup| {
        move |team: &Option<&Team>| {
            rsx! {
                td {
                    TeamSelect { group: group, team: team.map(|t| t.clone()) }
                } 
            }
        }
    }; 

    let card_count_selectors = tie_or_teams.iter().map(as_id_selector(PointsGroup::CardsCount));
    let coins_count_selectors = tie_or_teams.iter().map(as_id_selector(PointsGroup::CoinsCount));
    let settebello_selectors = teams.iter().map(|team| as_id_selector(PointsGroup::Settebello)(&Some(team)));
    let premiera_selectors = tie_or_teams.iter().map(as_id_selector(PointsGroup::Premiera));

    rsx! {
        table {
            tr { td { "" }             td { "- tied -" } { headers } }
            tr { th { "Total:"  }      td { "" }         { totals } }
            tr { td { "Scopas:"  }     td { "" }         { scopas } }
            tr { td { "Card count:" }  { card_count_selectors } }
            tr { td { "Coins count:" } { coins_count_selectors } }
            tr { td { "Settebello:" }  td { "" }         { settebello_selectors } }
            tr { td { "Premiera:" }    { premiera_selectors } }
        }
    }
}

#[component]
fn ScopaScore(
    team: Team,
    autofocus: bool
) -> Element {
    let id = team.id();
    let is_not_playing = team.is_not_playing();

    let mut round = use_context::<Signal<Round>>();

    let mut draft = use_signal(move || Points::default());

    use_effect(move || {
        draft.set(round.read().scopas(id));
    });

    let update_draft = move |points| {
        // draft.set(points);
        round.set(round().with_scopas(id, points));
    };

    // let update_string_draft = move |event: Event<FormData>| {
    //     if let Ok(points) = Points::try_from(event.value()) {
    //         draft.set(points);
    //         round.set(round().with_scopas(team.id(), points));
    //     }
    // };

    rsx! {
        PointsEditor {
            value: draft(),
            onchange: update_draft,
            autofocus: autofocus,
            disabled: is_not_playing,
        }
    }
}

#[component]
fn TeamSelect(
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

    rsx! {
        input {
            onchange: update_draft,
            r#type: "radio",
            name: group.to_string(),
            disabled: is_not_playing.unwrap_or(false),
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