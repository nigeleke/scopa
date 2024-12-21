use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};

#[component]
pub fn PointsView(value: Points) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/points.css") }
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

    let update_points = move |value: String| {
        let result = Points::try_from(value.clone());

        draft.set(value);

        if let Ok(value) = result {
            onchange.call(value);
        };
    };

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/points.css") }
        Container {
            label {
                Input {
                    value: value.to_string(),
                    typ: "number",
                    on_input: update_points,
                    min: "0",
                    autofocus: autofocus,
                    disabled: disabled,
                    aria_label,
                }
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
