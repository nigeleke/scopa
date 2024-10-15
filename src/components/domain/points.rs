use crate::domain::prelude::*;

use dioxus::prelude::*;

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
    #[props(default = false)]
    autofocus: bool,
    #[props(default = false)]
    disabled: bool,
) -> Element {

    let mut draft = use_signal(|| value.to_string());

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
                autofocus: autofocus,
                disabled: disabled,
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