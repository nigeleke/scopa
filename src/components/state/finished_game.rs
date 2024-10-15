use crate::components::prelude::Glow;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn FinishedGame(
    state: FinishedState,
    onchange: EventHandler<GameState>,
) -> Element {
    let winner = state.winner();

    let mut retain = use_signal(|| true);

    let update_retain = move |event: Event<FormData>| {
        retain.set(event.checked());
    };

    let start_new_game = move |_| {
        let mut new_game = Game::default();
        if retain() {
            new_game.set_target(state.target());
            let teams = state.teams();
            teams.iter().for_each(|t| { let _ = new_game.add_team(t.clone()); } );
        }
        onchange.call(new_game.start().unwrap());
    };

    rsx! {
        Glow {
            { "The winner is" }
            { winner.to_string() }
        }
        div {
            div {
                button {
                    onclick: start_new_game,
                    "New Game"
                }
            }
            input {
                r#type: "checkbox",
                checked: retain(),
                onchange: update_retain,
            }
            { " Retain players and target?" }
        }
    }
}
