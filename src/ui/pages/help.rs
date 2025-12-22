use dioxus::prelude::*;
use dioxus_i18n::{tid, unic_langid::langid};
use dioxus_primitives::scroll_area::ScrollDirection;

use crate::ui::{components::HomeIcon, i18n::Language, kit::scroll_area::ScrollArea, pages::Page};

#[component]
pub fn Help() -> Element {
    rsx! {
        div {
            class: "help",
            document::Stylesheet { href: asset!("/assets/css/pages/help.css") },
            div {}
            HelpContent {}
            div {}
        }
    }
}

#[component]
fn HelpContent() -> Element {
    let i18n = use_context::<Signal<Option<Language>>>();

    let mut region = use_signal(String::default);
    let mut starting_image = use_signal(String::default);
    let mut scoring_image = use_signal(String::default);

    use_effect(move || {
        region.set(
            i18n()
                .map_or(langid!("en-GB"), |l| l.identifier())
                .language
                .to_string(),
        );
        match region().as_str() {
            "it" => {
                starting_image.set(asset!("assets/images/starting-it.png").to_string());
                scoring_image.set(asset!("assets/images/scoring-it.png").to_string())
            }
            _ => {
                starting_image.set(asset!("assets/images/starting-en.png").to_string());
                scoring_image.set(asset!("assets/images/scoring-en.png").to_string())
            }
        }
    });

    let mut page = use_context::<Signal<Page>>();
    let on_home = move |_| page.set(Page::Home);

    rsx! {
        div {
            class: "help__content",
            ScrollArea {
                direction: ScrollDirection::Vertical,
                div {
                    class: "help__rules",
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
                    img {
                        class: "screenshot",
                        src: "{starting_image}"
                    }
                    p { {tid!("help.starting-points")} }
                    p { {tid!("help.starting-add-team")} }
                    p { {tid!("help.starting-remove-team")} }
                    p { {tid!("help.starting-start-game")} }
                    h2 { {tid!("help.scoring-heading")} }
                    p { {tid!("help.scoring-intro")} }
                    img {
                        class: "screenshot",
                        src: "{scoring_image}"
                    }
                    p { {tid!("help.scoring-scopa")} }
                    p { {tid!("help.scoring-basics")} }
                    p { {tid!("help.scoring-undo")} }

                }
            }
            HomeIcon { on_click: on_home, }
        }
    }
}
