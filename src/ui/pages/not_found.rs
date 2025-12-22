use dioxus::prelude::*;

use crate::ui::routes::Route;

#[component]
pub fn NotFound(_route: Vec<String>) -> Element {
    let navigator = use_navigator();

    use_effect(move || {
        navigator.replace(Route::Home);
    });

    rsx! {}
}
