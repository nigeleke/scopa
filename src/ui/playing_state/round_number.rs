use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::application::Model;

#[component]
pub fn RoundNumber() -> Element {
    let model = use_context::<Signal<Model>>();
    let round_number = model.read().round_number();

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_state/round_number.css") }
        div {
            class: "round-number",
            span {
                {tid!("round-view.text", n: round_number.to_string())}
            }
        }
    }
}
