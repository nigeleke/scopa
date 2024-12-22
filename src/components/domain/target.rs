use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_i18n::t;

#[component]
pub fn TargetView(value: Target) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/target.css") }
        Container {
            {t!("points-view-text", n: value.to_string())}
        }
    }
}

#[component]
pub fn TargetEditor(value: Target, onchange: EventHandler<Target>) -> Element {
    let update_target = move |value| {
        let result = Target::try_from(value);
        if let Ok(new_value) = result {
            onchange.call(new_value);
        };
    };

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/target.css") }
        Container {
            {t!("points-editor-prefix")}
            label {
                Input {
                    typ: "number",
                    value: value.to_string(),
                    on_input: update_target,
                    min: "11",
                    step: "5",
                    size: "3",
                    aria_label: t!("points-editor-aria-label"),
                }
            }
            {t!("points-editor-suffix")}
        }
    }
}

#[component]
fn Container(children: Element) -> Element {
    rsx! {
        div {
            class: "target-outer",
            div {
                class: "target-inner",
                {children},
            },
        }
    }
}
