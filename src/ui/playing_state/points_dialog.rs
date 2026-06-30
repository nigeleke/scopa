use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::Points;

#[component]
pub fn PointsDialog(on_change: EventHandler<Points>, on_cancel: EventHandler<()>) -> Element {
    let mut show_more = use_signal(|| false);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_state/points_dialog.css") }
        div {
            class: "points-dialog__overlay",
            div {
                class: "points-dialog",
                div {
                    class: "points-dialog__points",
                    for v in 0..=13 {
                        button {
                            hidden: v > 5 && !*show_more.read(),
                            aria_label: tid!("scopa-button.aria-label", n: v),
                            onclick: move |_| on_change.call(Points::from(v)),
                            "{v}"
                        }
                    }
                }

                div {
                    class: "points-dialog__controls",
                    button {
                        hidden: *show_more.read(),
                        aria_label: tid!("more-button.aria-label"),
                        onclick: move |_| show_more.set(true),
                        {tid!("more-button.text")}
                    }
                    button {
                        class: "points-dialog__cancel-button",
                        aria_label: tid!("cancel-button.aria-label"),
                        onclick: move |_| on_cancel.call(()),
                        {tid!("cancel-button.text")}
                    }
                }
            }
        }
    }
}
