use crate::types::Points;

use dioxus::prelude::*;

#[component]
pub fn TargetView(
    value: Points,
) -> Element {
    rsx! {
        { value.to_string() }
    }
}

#[component]
pub fn TargetEditor (
    value: Signal<Points>,
    #[props(extends = input)]
    attributes: Vec<Attribute>,
 ) -> Element {

    let update_target = move |event: Event<FormData>| {
        let result = Points::try_from(event.value());
        if let Ok(new_value) = result {
            value.set(new_value);
        };
    };

    rsx! {
        input {
            value: value.to_string(),
            oninput: update_target,
            r#type: "number",
            min: "11",
            step: "5",
        }
    }
}
