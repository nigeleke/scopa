use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_sdk::storage::{use_storage, LocalStorage};

#[component]
pub fn Home() -> Element {
    let mut game = use_storage::<LocalStorage, _>("game".into(), GameState::default);
    provide_context(game);

    let update_game = move |new_game| {
        game.set(new_game);
    };

    rsx! {
        Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
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
