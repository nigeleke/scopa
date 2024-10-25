use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn TargetView(
    value: Target,
) -> Element {
    rsx! {
        Container {
            "Playing to "
            { value.to_string() }
            " points"
        }
    }
}

#[component]
pub fn TargetEditor (
    value: Target,
    onchange: EventHandler<Target>,
 ) -> Element {

    let update_target = move |value| {
        let result = Target::try_from(value);
        if let Ok(new_value) = result {
            onchange.call(new_value);
        };
    };

    rsx! {
        Container {
            "Play to"
            label {
                Input {
                    typ: "number",
                    value: value.to_string(),
                    on_input: update_target,
                    min: "11",
                    step: "5",
                    size: "3",
                    aria_label: "Enter game target (default 11)",
                }
            }
            "points"
        }
    }
}

#[component]
fn Container(
    children: Element,
) -> Element {
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
