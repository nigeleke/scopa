use dioxus::prelude::*;

use crate::ui::{
    components::{Errors, FinishedGame, PlayingGame, StartingGame},
    state::State,
};

#[component]
pub fn Home() -> Element {
    let mut state = use_context::<Signal<State>>();

    let update_state = move |new_state| {
        state.set(new_state);
    };

    rsx! {
        ErrorBoundary {
            handle_error: |context| rsx! { Errors { context } },
            match state() {
                State::Starting(game) => rsx! { StartingGame { game, onchange: update_state, } },
                State::Playing(game) => rsx! { PlayingGame { game, onchange: update_state } },
                State::Finished(game) => rsx! { FinishedGame { game, onchange: update_state } },
            }
        }
    }
}
