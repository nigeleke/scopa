use dioxus::prelude::*;

use crate::{domain::Points, ui::playing_state::points_dialog::PointsDialog};

#[component]
pub fn PointsEditor(
    value: Points,
    on_change: EventHandler<Points>,
    #[props(default = false)] autofocus: bool,
    #[props(default = false)] disabled: bool,
    aria_label: String,
) -> Element {
    let mut show_dialog = use_signal(|| false);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_state/points_editor.css") }
        div {
            class: "points-editor",
            button {
                class: "points-editor__points-button",
                disabled,
                aria_label,
                autofocus,
                tabindex: "0",
                onclick: move |_| show_dialog.set(true),
                "{value}"
            }
            if *show_dialog.read() {
                PointsDialog {
                    on_change: move |value| {
                        show_dialog.set(false);
                        on_change.call(value);
                    },
                    on_cancel: move |_| show_dialog.set(false)
                }
            }
        }
    }
}
