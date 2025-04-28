use dioxus::prelude::*;
use dioxus_i18n::tid;

use super::{
    button::Button, icon::UndoIcon, round::RoundEditor, round_number::RoundNumberView,
    target::TargetView,
};
use crate::{domain::*, ui::state::State};

#[component]
pub fn PlayingGame(game: ReadOnlySignal<Game<Playing>>, onchange: EventHandler<State>) -> Element {
    let round_number = game.read().history().round_number();
    let target = game.read().target();

    let undo = move |_| {
        let playing_state = game().undo()?;
        onchange.call(State::from(playing_state));
        Ok(())
    };

    let mut round = use_signal(Round::default);

    let score_round = move |_| {
        let result = game().score_round(&round.read())?;
        match result {
            ScoreRoundResult::Playing(game) => onchange.call(State::from(game)),
            ScoreRoundResult::Finished(game) => onchange.call(State::from(game)),
        }
        round.set(Round::default());
        Ok(())
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/pages/playing_game.css") }
        div {
            class: "playing-game",
            div {
                class: "playing-game-header",
                RoundNumberView { value: round_number, }
                TargetView { value: target, }
            }
            RoundEditor {
                game: game(),
                round: round,
            }
            div {
                class: "playing-game-controls",
                UndoIcon {
                    can_undo: game().can_undo(),
                    on_click: undo,
                }
                ScoreButton {
                    can_score: round.read().is_well_defined(),
                    onscore: score_round,
                }
            }
        }
    }
}

#[component]
fn ScoreButton(can_score: bool, onscore: EventHandler<()>) -> Element {
    let record_score = move |_| onscore.call(());

    rsx! {
        Button {
            id: "score",
            disabled: !can_score,
            on_click: record_score,
            {tid!("score-button.text")}
        }
    }
}
