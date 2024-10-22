use dioxus::prelude::*;

use super::prelude::PointsGroup;

#[component]
pub fn CardsImage(
    group: PointsGroup,
    disabled: bool,
    checked: bool,
) -> Element {
    let src = match group {
        PointsGroup::CardsCount => asset!("./assets/images/punteggio_carte.png"),
        PointsGroup::CoinsCount => asset!("./assets/images/punteggio_denari.png"),
        PointsGroup::Settebello => asset!("./assets/images/punteggio_settebello.png"),
        PointsGroup::Premiera => asset!("./assets/images/punteggio_premiere.png"),
    };

    rsx! {
        div {
            class: "cards-image",
            class: if disabled { "disabled" },
            class: if checked { "checked" }, 
            img {
                src: src,
            }
        } 
    }
}
