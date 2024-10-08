use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut game = use_context_provider(|| Signal::new(GameState::default()));
    
    let update_game = move |new_game| game.set(new_game);

    rsx! {
        header { ScopaHeader {} }
        main {
            match game() {
                GameState::Starting(game) => rsx! { StartingGame { game, onchange: update_game, } },
                GameState::Playing(game) => rsx! { PlayingGame { game, onchange: update_game } },
                GameState::Finished(game) => rsx! { FinishedGame { game, onchange: update_game } },
            }
        }
        footer { ScopaFooter {} }
    }
}
