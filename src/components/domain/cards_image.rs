use dioxus::prelude::*;

use super::prelude::PointsGroup;

#[component]
pub fn CardsImage(
    group: PointsGroup,
    disabled: bool,
    checked: bool,
) -> Element {
    let src = match group {
        PointsGroup::CardsCount => asset!("./assets/images/cards.png"),
        PointsGroup::CoinsCount => asset!("./assets/images/coins.png"),
        PointsGroup::Settebello => asset!("./assets/images/settebello.png"),
        PointsGroup::Premiera => asset!("./assets/images/premiere.png"),
    };

    rsx! {
        div {
            class: "cards-icon",
            class: if disabled { "disabled" },
            class: if checked { "checked" }, 
            img {
                src: src,
            }
        } 
    }
}
