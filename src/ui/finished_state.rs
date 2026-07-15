use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::application::Model;
use crate::ui::glow::Glow;
use crate::ui::icon_button::{Icon, IconButton};

#[component]
pub fn FinishedState() -> Element {
    let mut model = use_context::<Signal<Model>>();
    let winner = model
        .read()
        .winner()
        .cloned()
        .expect("must have winner to be finished");

    let mut teams = Vec::from_iter(model.read().teams().cloned());
    teams.sort_by_key(|t| model.read().score(*t.id()));
    teams.reverse();

    let mut retain_settings = use_signal(|| true);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/finished_state.css") },
        div {
            class: "finished-state",
            div {
                class: "finished-state__winner",
                Glow { {tid!("winner-view.text", teamname: winner.name().to_string())} }
            }
            div {
                class: "finished-state__placings",
                for team in teams {
                    span { "{team.name()}" }
                    span { "{model.read().score(*team.id())}" }
                }
            }
            div {
                class: "finished-state__actions",
                IconButton {
                    icon: Icon::Undo,
                    class: "finished-state__undo-button",
                    title: tid!("undo-button.hint"),
                    aria_label: tid!("undo-button.aria-label"),
                    on_click: move || {
                        model.write().remove_round();
                        model.write().play();
                    },
                }
                IconButton {
                    icon: Icon::Reset,
                    class: "finished-state__restart-button",
                    title: tid!("restart-button.text"),
                    aria_label: tid!("restart-button.aria-label"),
                    on_click: move || {
                        if *retain_settings.read() {
                            model.write().start();
                        } else {
                            model.set(Model::default());
                        }
                    },
                }
                label {
                    input {
                        r#type: "checkbox",
                        aria_label: tid!("restart-settings.aria-label"),
                        checked: *retain_settings.read(),
                        onclick: move |_| {
                            let retain = !*retain_settings.read();
                            retain_settings.set(retain)
                        },
                    }
                    span { {tid!("restart-settings.text")} }
                }
            }
        }
    }
}
