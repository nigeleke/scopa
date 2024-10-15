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
    #[props(extends = input)]
    attributes: Vec<Attribute>,
 ) -> Element {

    let update_target = move |event: Event<FormData>| {
        let result = Target::try_from(event.value());
        if let Ok(new_value) = result {
            onchange.call(new_value);
        };
    };

    rsx! {
        Container {
            "Play to"
            input {
                value: value.to_string(),
                oninput: update_target,
                r#type: "number",
                min: "11",
                step: "5",
                size: "3",
                ..attributes,
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
