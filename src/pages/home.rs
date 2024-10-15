use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut game = use_signal(|| GameState::default());
    
    let update_game = move |new_game| {
        game.set(new_game);
    };

    rsx! {
        header { ScopaHeader {} }
        main {
            match game() {
                GameState::Starting(game) => rsx! { StartingGame { state: game.state(), onchange: update_game, } },
                GameState::Playing(game) => rsx! { PlayingGame { state: game.state(), onchange: update_game } },
                GameState::Finished(game) => rsx! { FinishedGame { state: game.state(), onchange: update_game } },
            }
        }
        footer { ScopaFooter {} }
    }
}
