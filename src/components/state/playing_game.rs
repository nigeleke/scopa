use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn PlayingGame(
    game: Game<PlayingState>,
    onchange: EventHandler<GameState>
) -> Element {

    let game = use_memo(move || game.clone());
    use_context_provider(move || game());

    let mut round = use_signal(move || Round::default());

    let update_round = move |new_round| round.set(new_round);

    let score_round = move |_| {
        let new_game = game().score_round(&round.read()).unwrap();
        onchange.call(new_game);
    };

    rsx! {
        div {
            class: "playing-game",
            TargetView { value: game.read().target(), }
            RoundNumberView { value: game.read().round_number(), }
            RoundEditor {
                teams: Vec::from(game.read().teams()),
                round: round(),
                onchange: update_round,
            }
            ScoreButton {
                can_score: round.read().is_well_defined(),
                onscore: score_round, 
            }
            }
    }
}

#[component]
fn ScoreButton(
    can_score: bool,
    onscore: EventHandler<()>
) -> Element {

    let record_score = move |_| {
        onscore.call(());
    };

    rsx! {
        button {
            disabled: !can_score,
            onclick: record_score,
            "Score"
        }
    }
}