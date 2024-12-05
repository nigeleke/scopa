use crate::components::prelude::*;
use crate::domain::prelude::*;
use crate::use_persistent::use_persistent;

use dioxus::prelude::document::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut game = use_persistent("game", GameState::default);

    let update_game = move |new_game| {
        game.set(new_game);
    };

    rsx! {
        Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        Link { rel: "stylesheet", href: asset!("/assets/css/main.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/points.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/points_group_image.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/round_number.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/round.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/scopa_header.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/target.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/state/finished_game.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/state/playing_game.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/state/starting_game.css") }
        Link { rel: "stylesheet", href: asset!("/assets/css/ui/glow.css") }
        header { ScopaHeader {} }
        main {
            match game.get() {
                GameState::Starting(game) => rsx! { StartingGame { state: game.state(), onchange: update_game, } },
                GameState::Playing(game) => rsx! { PlayingGame { state: game.state(), onchange: update_game } },
                GameState::Finished(game) => rsx! { FinishedGame { state: game.state(), onchange: update_game } },
            }
        }
        footer { ScopaFooter {} }
    }
}
