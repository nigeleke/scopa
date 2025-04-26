use dioxus::prelude::*;
use dioxus_i18n::tid;

use super::button::Button;
use crate::{domain::Game, ui::state::State};

#[component]
pub fn Errors(context: ErrorContext) -> Element {
    let mut state = use_context::<Signal<State>>();
    let mut context = use_signal(|| context);

    let onreset = move |_| {
        let game = Game::default();
        state.set(State::from(game));
        context.write().clear_errors();
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
            for (i, error) in context.read().errors().iter().enumerate() {
                li { key: "{i}", "{error}" }
            }
        }
        Button { on_click: onreset, {tid!("reset-button.text")} }
    }
}
