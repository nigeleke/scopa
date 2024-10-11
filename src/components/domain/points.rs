use crate::types::Points;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn PointsView(
    value: Points
) -> Element {
    rsx! {
        span { { value.to_string() } }
    }
}

#[component]
pub fn PointsEditor(
    value: Points,
    onchange: EventHandler<Points>,
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {

    let mut draft = use_signal(|| value.into());

    let update_points = move |event: Event<FormData>| {
        draft.set(event.value());

        let result = Points::try_from(event.value());
        if let Ok(value) = result {
            onchange.call(value);
        };
    };

    rsx! {
        Container {
            input {
                value: value.to_string(),
                oninput: update_points,
                r#type: "number",
                min: "0",
                ..attributes,
            }
        }
    }
}

#[component]
fn Container(
    children: Element
) -> Element {
    rsx! {
        div {
            class: "points",
            {children}
        }
    }
}