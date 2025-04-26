use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_i18n::tid;

use super::input::Input;
use crate::domain::*;

#[component]
pub fn TargetView(value: Target) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/target.css") }
        Container {
            {tid!("points-view.text", n: value.to_string())}
        }
    }
}

#[component]
pub fn TargetEditor(value: Target, onchange: EventHandler<Target>) -> Element {
    let update_target = move |value: String| {
        let result = Target::from_str(value.as_str());
        if let Ok(new_value) = result {
            onchange.call(new_value);
        };
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/target.css") }
        Container {
            {tid!("points-editor.prefix")}
            label {
                Input {
                    typ: "number",
                    value: "{value}",
                    on_input: update_target,
                    min: "11",
                    step: "5",
                    size: "3",
                    aria_label: tid!("points-editor.aria-label"),
                }
            }
            {tid!("points-editor.suffix")}
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
