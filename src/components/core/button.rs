use dioxus::prelude::*;

#[component]
pub fn Button(
    on_click: EventHandler<()>,

    #[props(default = "0".into())]
    tabindex: String,

    #[props(optional)]
    children: Element,

    #[props(extends = button)]
    attributes: Vec<Attribute>,
) -> Element {

    rsx! {
        button {
            onclick: move |_| on_click(()),
            onkeydown: move |event| if event.key() == Key::Enter { on_click(()) },
            tabindex,
            ..attributes,
            {children}
        }
    }

} 