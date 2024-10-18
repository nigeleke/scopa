use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut game = use_signal(GameState::default);
    
    let update_game = move |new_game| {
        game.set(new_game);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/css/main.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/domain/cards_icon.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/domain/points.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/domain/round_number.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/domain/round.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/domain/scopa_header.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/domain/target.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/state/finished_game.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/state/playing_game.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/state/starting_game.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/ui/glow.css") }
        document::Link { rel: "stylesheet", href: asset!("./assets/css/ui/icon.css") }
        document::Script { src: asset!(file("./assets/js/elements.cardmeister.min.js")) }
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
