use dioxus::prelude::*;
use dioxus_i18n::tid;
use gloo_timers::future::sleep;

use crate::domain::*;

#[component]
pub fn RoundNumberView(value: ReadSignal<RoundNumber>) -> Element {
    let mut pulse = use_signal(|| false);

    use_effect(move || {
        let _ = value();
        pulse.set(true);
        spawn(async move {
            sleep(std::time::Duration::from_millis(1000)).await;
            pulse.set(false);
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/round_number.css") }
        Container {
            pulse: pulse(),
            {tid!("round-view.text")}
            { " " }
            { value.to_string() }
        }
    }
}

#[component]
fn Container(pulse: bool, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/round_number.css") }
        div {
            class: "round-number-outer",
            class: if pulse { "round-number-pulse" },
            div {
                class: "round-number-inner",
                {children},
            },
        }
    }
}
