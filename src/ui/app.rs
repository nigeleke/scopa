use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::tid;

use crate::application::Page;
use crate::i18n::{self, Language};
use crate::storage;
use crate::ui::game_page::GamePage;
use crate::ui::glow::Glow;
use crate::ui::help_page::HelpPage;
use crate::ui::main_menu::MainMenu;

#[component]
pub fn App() -> Element {
    let mut i18n = use_init_i18n(|| i18n::config(Language::english().identifier()));

    let model = storage::use_application_model();
    provide_context(model);

    use_effect(move || {
        if let Some(language) = model().language() {
            i18n.set_language(language.identifier());
        }
    });

    rsx! {
        document::Title { {tid!("scopa-app.title-text")} }
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Stylesheet { href: asset!("/assets/css/style.css") }
        document::Meta { name: "description", content: "Score your Scopa game using this website. This app is a free and easy to use program to help you score your Scopa card game." }
        document::Meta { name: "category", content: "game" }
        document::Meta { name: "keywords", content: "cards,game,scorer,scopa" }
        document::Meta { name: "author", content: "Nigel Eke" }

        Layout {
            match model().page() {
                Page::Help => rsx! { HelpPage {} },
                Page::Game => rsx! { GamePage {} },
            }
        }
    }
}

#[component]
fn Layout(children: Element) -> Element {
    let version = env!("CARGO_PKG_VERSION");
    rsx! {
        header {  Glow { {tid!("scopa-app.title-text")} } }
        // main {
        //     MainMenu {}
        //     { children }
        // }
        // footer {
        //     { tid!("scopa-app.version-text", version: version) }
        //     " "
        //     { tid!("scopa-app.author-text") }
        //     " "
        //     a {
        //         href: "https://opensource.org/license/bsd-3-clause",
        //         target: "_blank",
        //         rel: "noopener noreferrer",
        //         { tid!("scopa-app.license-text") }
        //     }
        // }
    }
}
