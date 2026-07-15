use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::application::Model;
use crate::ui::icon_button::{Icon, IconButton};

#[component]
pub fn HelpPage() -> Element {
    let mut model = use_context::<Signal<Model>>();

    rsx! {
        div {
            class: "help-page",
            document::Stylesheet { href: asset!("/assets/css/help_page.css") },
            HelpContent {}
            HomeButton { on_game: move |_| model.write().show_game() }
        }
    }
}

#[component]
fn HelpContent() -> Element {
    rsx! {
        div {
            class: "help-page__content",
            div {
                class: "help-page__rules",
                h1 { {tid!("help.heading")} }
                p { {tid!("help.intro")} }
                h2 { {tid!("help.rules-heading")} }
                p { {tid!("help.rules-teams")} }
                p { {tid!("help.rules-deal")} }
                p { {tid!("help.rules-aim")} }
                p { {tid!("help.rules-play-1")} }
                p { {tid!("help.rules-play-2")} }
                p { {tid!("help.rules-play-3")} }
                p { {tid!("help.rules-play-4")} }
                p { {tid!("help.rules-play-5")} }
                p { {tid!("help.rules-play-6")} }
                h2 { {tid!("help.starting-heading")} }
                p { {tid!("help.starting-intro")} }
                Screenshot { name: "starting" }
                p { {tid!("help.starting-points")} }
                p { {tid!("help.starting-add-team")} }
                p { {tid!("help.starting-remove-team")} }
                p { {tid!("help.starting-start-game")} }
                h2 { {tid!("help.scoring-heading")} }
                p { {tid!("help.scoring-intro")} }
                Screenshot { name: "scoring" }
                p { {tid!("help.scoring-scopa")} }
                p { {tid!("help.scoring-basics")} }
                p { {tid!("help.scoring-undo")} }
            }
        }
    }
}

#[component]
fn Screenshot(name: String) -> Element {
    let model = use_context::<Signal<Model>>();
    let root = model().language().unwrap_or_default().root_string();

    rsx! {
        img { class: "help-page__screenshot", src: "assets/images/{name}-{root}.png" }
    }
}

#[component]
fn HomeButton(on_game: EventHandler<()>) -> Element {
    rsx! {
        IconButton {
            icon: Icon::Home,
            title: tid!("home-button.title"),
            aria_label: tid!("home-button.aria-label"),
            on_click: move |_| on_game(()),
        }
    }
}
