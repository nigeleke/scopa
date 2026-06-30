use dioxus::prelude::*;
use dioxus_i18n::*;

use super::Group;

#[component]
pub fn GroupImage(hint: Option<String>, group: Group, disabled: bool, checked: bool) -> Element {
    let src = match group {
        Group::Scopa => asset!("/assets/images/punteggio_scopa.png"),
        Group::CardsCount => asset!("/assets/images/punteggio_carte.png"),
        Group::CoinsCount => asset!("/assets/images/punteggio_denari.png"),
        Group::Settebello => asset!("/assets/images/punteggio_settebello.png"),
        Group::Premiera => asset!("/assets/images/punteggio_premiera.png"),
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_state/group_image.css") }
        div {
            class: "group-image",
            class: if disabled { "disabled" },
            class: if checked { "checked" },
            img {
                src: src,
                alt: tid!(&format!("{group}-icon.alt-text")),
            }
            if let Some(hint) = hint {
                div { "{hint.to_string()}" }
            }
        }
    }
}
