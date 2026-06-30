use dioxus::prelude::*;

use crate::application::Model;
use crate::domain::GameState;

use crate::ui::finished_state::FinishedState;
use crate::ui::playing_state::PlayingState;
use crate::ui::setup_state::SetupState;

#[component]
pub fn GamePage() -> Element {
    let mut model = use_context::<Signal<Model>>();

    use_effect(move || {
        let state = model.read().state();
        match state {
            GameState::Playing => {
                let leading_teams = model.read().leading_teams().cloned().collect::<Vec<_>>();

                if leading_teams.len() == 1 {
                    let target = model.read().target();
                    let id = *leading_teams[0].id();

                    if model.read().score(id).value() >= target.value() {
                        model.write().finish();
                    }
                }
            }
            _ => {}
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/game_page.css") }
        match model.read().state() {
            GameState::Setup => rsx! { SetupState {} },
            GameState::Playing => rsx! { PlayingState {} },
            GameState::Finished => rsx! { FinishedState {} },
        }
    }
}
