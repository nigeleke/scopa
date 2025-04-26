use dioxus::prelude::*;
use dioxus_i18n::tid;

use super::points_group::PointsGroup;

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
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/points_group_image.css") }
        div {
            class: "points-group-image",
            class: if disabled { "disabled" },
            class: if checked { "checked" },
            img {
                src: src,
                alt: tid!(&format!("{}-icon.alt-text", group)),
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
