use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn FinishedGame(
    game: Game<FinishedState>,
    onchange: EventHandler<GameState>,
) -> Element {
    let winner = game.winner();

    let mut retain = use_signal(|| true);

    let update_retain = move |event: Event<FormData>| {
        retain.set(event.checked());
    };

    let start_new_game = move |_| {
        let mut new_game = Game::default();
        if retain() {
            new_game.set_target(game.target());
            let teams = game.teams();
            teams.iter().for_each(|t| { let _ = new_game.add_team(t.clone()); } );
        }
        onchange.call(new_game.start().unwrap());
    };

    rsx! {
        { "The winner is" }
        { winner.to_string() }
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
