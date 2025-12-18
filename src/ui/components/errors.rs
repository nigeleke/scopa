use dioxus::prelude::*;
use dioxus_i18n::tid;

use super::button::Button;
use crate::{domain::Game, ui::state::State};

#[component]
pub fn Errors(errors: ErrorContext) -> Element {
    let mut state = use_context::<Signal<State>>();

    let onreset = move |_| {
        let game = Game::default();
        state.set(State::from(game));
    };

    rsx! {
        h2 { {tid!("error.apology")} }
        p {
            {tid!("error.report0")}
            {" "}
            Link { to: "https://github.com/nigeleke/scopa/issues", {tid!("error.report1")} }
            {" "}
            {tid!("error.report2")}
        }
        ul {
            if let Some(error) = errors.error() {
                li { {error.to_string()} }
            }
        }
        Button { on_click: onreset, {tid!("reset-button.text")} }
    }
}
