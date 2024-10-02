use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct InputProps<
    T: dioxus::prelude::Properties + Into<String> + TryFrom<String> + Default + std::fmt::Debug + PartialEq,
> {
    value: T,
    onchange: Option<EventHandler<T>>,
    oncommit: Option<EventHandler<T>>,
    onerror: Option<EventHandler<<T as TryFrom<String>>::Error>>,
    #[props(extends = input)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Input<
    T: dioxus::prelude::Properties + Into<String> + TryFrom<String> + Default + std::fmt::Debug + PartialEq,
> (props: InputProps<T>) -> Element
where 
    <T as TryFrom<String>>::Error: Clone + std::fmt::Debug + PartialEq
{
    let mut draft = use_signal(move || props.value.into());
    let mut is_valid = use_signal(move || true);

    let oninput = move |event: Event<FormData>| { 
        draft.set(event.value());
        let result = T::try_from(draft());
        dioxus_logger::tracing::info!("Input::oninput {:?} => {:?}", event, result);
        is_valid.set(result.is_ok());
        match result {
            Ok(new_value) => {
                dioxus_logger::tracing::info!("Input calling onchange handler {}", new_value.clone().into());
                props.onchange.map(|handler| handler.call(new_value));
            },
            Err(e) => { props.onerror.map(|handler| handler.call(e)); },
        };
    };

    let onkeydown = move |event: KeyboardEvent| {
        dioxus_logger::tracing::info!("On keydown {:?} => is_valid: {}", event, is_valid());
        if event.key() == Key::Enter && is_valid() {
            dioxus_logger::tracing::info!("Input::onkeydown {:?} ", event);
            let new_value = T::try_from(draft()).unwrap();
            props.oncommit.map(|handler| handler(new_value));
        }
    };

    rsx! {
        input {
            class: if !is_valid() { "invalid" },
            oninput: oninput,
            onkeydown: onkeydown,
            value: draft(),
            ..props.attributes,
        }
    }
}
