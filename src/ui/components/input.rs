use dioxus::prelude::*;

#[component]
pub fn Input(
    value: String,
    typ: String,
    on_input: EventHandler<String>,
    on_commit: Option<EventHandler<()>>,
    aria_label: String,
    #[props(default = "0".into())] tabindex: String,
    #[props(extends = input)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        input {
            value,
            r#type: typ,
            oninput: move |event| on_input.call(event.value()),
            onkeydown: move |event: KeyboardEvent| if event.key() == Key::Enter { if let Some(f) = on_commit { f.call(()) } },
            aria_label,
            tabindex,
            ..attributes,
        }
    }
}
