use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_sdk::storage::{use_storage, LocalStorage};

#[component]
pub fn Home() -> Element {
    let default_target = use_storage::<LocalStorage, _>("default_target".into(), Target::default);
    let mut game =
        use_storage::<LocalStorage, _>("game".into(), || GameState::new(default_target()));
    provide_context(game);

    let update_game = move |new_game| {
        game.set(new_game);
    };

    rsx! {
        Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
        Meta { name: "description", content: "Score your Scopa game using this website. This app is a free and easy to use program to help you score your Scopa card game." }
        Meta { name: "category", content: "game" }
        Meta { name: "keywords", content: "cards,game,scorer,scopa" }
        Meta { name: "author", content: "Nigel Eke" }
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
