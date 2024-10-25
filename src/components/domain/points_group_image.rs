use dioxus::prelude::*;

use super::prelude::PointsGroup;

#[component]
pub fn PointsGroupImage(
    group: PointsGroup,
    disabled: bool,
    checked: bool,
) -> Element {
    let src = match group {
        PointsGroup::Scopa => asset!("./assets/images/punteggio_scopa.png"),
        PointsGroup::CardsCount => asset!("./assets/images/punteggio_carte.png"),
        PointsGroup::CoinsCount => asset!("./assets/images/punteggio_denari.png"),
        PointsGroup::Settebello => asset!("./assets/images/punteggio_settebello.png"),
        PointsGroup::Premiera => asset!("./assets/images/punteggio_premiere.png"),
    };

    rsx! {
        div {
            class: "points-group-image",
            class: if disabled { "disabled" },
            class: if checked { "checked" }, 
            img {
                src: src,
                alt: format!("{} icon", group.to_string()),
            }
        } 
    }
}
