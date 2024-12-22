use dioxus::prelude::{document::*, *};
use dioxus_i18n::t;

use super::prelude::PointsGroup;

#[component]
pub fn PointsGroupImage(
    hint: Option<String>,
    group: PointsGroup,
    disabled: bool,
    checked: bool,
) -> Element {
    let src = match group {
        PointsGroup::Scopa => asset!("/assets/images/punteggio_scopa.png"),
        PointsGroup::CardsCount => asset!("/assets/images/punteggio_carte.png"),
        PointsGroup::CoinsCount => asset!("/assets/images/punteggio_denari.png"),
        PointsGroup::Settebello => asset!("/assets/images/punteggio_settebello.png"),
        PointsGroup::Premiera => asset!("/assets/images/punteggio_premiera.png"),
    };

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/points_group_image.css") }
        div {
            class: "points-group-image",
            class: if disabled { "disabled" },
            class: if checked { "checked" },
            img {
                src: src,
                alt: t!(&format!("{}-icon-alt-text", group)),
            }
            if let Some(hint) = hint {
                span {
                    class: "points-group-image-hint",
                    "{hint}"
                }
            }
        }
    }
}
