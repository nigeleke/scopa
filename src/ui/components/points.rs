use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{domain::*, ui::kit::dialog::*};

#[component]
pub fn PointsView(value: Points) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/points.css") }
        span { { value.to_string() } }
    }
}

#[component]
pub fn PointsEditor(
    value: Points,
    onchange: EventHandler<Points>,
    #[props(default = false)] autofocus: bool,
    #[props(default = false)] disabled: bool,
    aria_label: String,
) -> Element {
    let mut draft = use_signal(|| value.to_string());

    let mut show_dialog = use_signal(|| false);
    let mut show_more = use_signal(|| false);

    let mut select_value = move |value: usize| {
        draft.set(value.to_string());
        show_dialog.set(false);
        show_more.set(false);
        onchange.call(Points::from(value));
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/points.css") }
        Container {
            button {
                class: "points__button",
                disabled,
                aria_label,
                autofocus,
                onclick: move |_| show_dialog.set(true),
                "{value}"
            }

            DialogRoot {
                open: show_dialog(),
                DialogDescription {
                    class: "points__dialog",
                    div {
                        class: "points__buttons",
                        for v in 0..=13 {
                            button {
                                class: "points__button",
                                hidden: v > 5 && !show_more(),
                                onclick: move |_| select_value(v),
                                "{v}"
                            }
                        }
                    }
                    div {
                        class: "points__commands",
                        button {
                            hidden: show_more(),
                            onclick: move |_| show_more.set(true),
                            {tid!("more-button.text")}
                        }

                        button {
                            class: "points__cancel-button",
                            onclick: move |_| {
                                show_dialog.set(false);
                                show_more.set(false);
                            },
                            {tid!("cancel-button.text")}
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn Container(children: Element) -> Element {
    rsx! {
        div {
            class: "points",
            {children}
        }
    }
}
