use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_i18n::tid;

#[component]
pub fn PlayingGame(state: PlayingState, onchange: EventHandler<GameState>) -> Element {
    let round_number = state.round_number();
    let target = state.target();

    let mut round = use_signal(Round::default);

    let current_state = state.clone();
    let score_round = move |_| {
        let new_game = current_state.score_round(&round.read()).unwrap();
        onchange.call(new_game);
        round.set(Round::default());
    };

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/state/playing_game.css") }
        div {
            class: "playing-game",
            div {
                class: "playing-game-header",
                RoundNumberView { value: round_number, }
                TargetView { value: target, }
            }
            RoundEditor {
                state: state,
                round: round,
            }
            ScoreButton {
                can_score: round.read().is_well_defined(),
                onscore: score_round,
            }
        }
    }
}

#[component]
fn ScoreButton(can_score: bool, onscore: EventHandler<()>) -> Element {
    let record_score = move |_| {
        onscore.call(());
    };

    rsx! {
        Button {
            disabled: !can_score,
            on_click: record_score,
            {tid!("score-button.text")}
        }
    }
}
