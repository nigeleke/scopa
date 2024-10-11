use dioxus::prelude::*;

#[component]
pub fn ScopaFooter() -> Element {
    rsx! {
        p { "Copyright © 2024; Nigel Eke. All rights reserved." }
        p { "Acknowledgements: "
            a {
                href: "https://www.flaticon.com/free-icons/clean",
                title: "clean icons",
                "Clean icons created by Freepik - Flaticon"
            }
        }
    }
}