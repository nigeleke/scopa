use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{
    domain::Game,
    ui::{components::button::Button, pages::Page, state::State},
};

#[component]
pub fn Errors(errors: ErrorContext) -> Element {
    let mut state = use_context::<Signal<State>>();
    let mut page = use_context::<Signal<Page>>();

    let onreset = move |_| {
        let game = Game::default();
        state.set(State::from(game));
        page.set(Page::Home);
    };

    rsx! {
        h2 { {tid!("error.apology")} }
        p {
            {tid!("error.report0")}
            {" "}
            a { href: "https://github.com/nigeleke/scopa/issues", {tid!("error.report1")} }
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
