use crate::components::prelude::{StartingGame, PlayingGame, FinishedGame};
use crate::domain::*;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let game = use_context_provider(|| Signal::new(Game::new()));

    rsx! {
        header { "Scopa Scorer" }
        main {
            if game.read().is_starting() {
                StartingGame {}
            } else if game.read().is_playing() {
                PlayingGame {}
            } else if game.read().is_finished() {
                FinishedGame {}
            }
        }
        footer { "Copyright © 2024; Nigel Eke. All rights reserved." }
    }
}
