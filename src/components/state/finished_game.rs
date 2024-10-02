use crate::domain::*;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn FinishedGame() -> Element {
    let mut context = use_context::<Signal<Game>>();

    let winner = context.read().winner().unwrap();

    let mut retain = use_signal(|| true);

    let update_retain = move |event: Event<FormData>| {
        info!("event: {:?}", event);
        retain.set(event.checked());
    };

    let start_new_game = move |_| {
        let mut new_game = Game::new();
        if retain() {
            let teams = context.read().teams();
            let names = teams.iter().map(Team::name);
            names.for_each(|t| { let _ = new_game.add_team(t.to_string().as_str()); } );
            new_game = new_game.start(context.read().target()).unwrap();
        }
        context.set(new_game);
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
