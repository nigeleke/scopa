use dioxus::prelude::*;

use crate::application::Model;
use crate::domain::GameState;

use crate::ui::finished_state::FinishedState;
use crate::ui::playing_state::PlayingState;
use crate::ui::setup_state::SetupState;

#[component]
pub fn GamePage() -> Element {
    let model = use_context::<Signal<Model>>();

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/game_page.css") }
        match model.read().state() {
            GameState::Setup => rsx! { SetupState {} },
            GameState::Playing => rsx! { PlayingState {} },
            GameState::Finished => rsx! { FinishedState {} },
        }
    }
}
