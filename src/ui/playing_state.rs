mod group;
mod group_image;
mod group_radio_button;
mod points_dialog;
mod points_editor;
mod round_editor;
mod round_number;
mod scopa_icon;
mod scopa_score;
mod team_header;

use group::Group;
use group_image::GroupImage;
use group_radio_button::GroupRadioButton;
use points_editor::PointsEditor;
use round_editor::RoundEditor;
use round_number::RoundNumber;
use scopa_icon::ScopaIcon;
use scopa_score::ScopaScore;
use team_header::TeamHeader;

use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::application::Model;
use crate::domain::{Round, RoundNumber as DomainRoundNumber};
use crate::ui::icon_button::IconButton;

#[component]
pub fn PlayingState() -> Element {
    let mut model = use_context::<Signal<Model>>();

    let mut round = use_signal(Round::default);
    provide_context(round);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_state.css") },
        div {
            class: "playing-state",
            RoundEditor {  }
            div {
                class: "playing-state__actions",
                IconButton {
                    icon: "\u{21a9}",
                    class: "playing-state__undo-button",
                    title: tid!("undo-button.hint"),
                    aria_label: tid!("undo-button.aria-label"),
                    disabled: model.read().round_number() == DomainRoundNumber::one(),
                    on_click: move |_| {
                        round.set(model.read().last_round().cloned().unwrap_or_default());
                        model.write().remove_round();
                    },
                }
                button {
                    aria_label: tid!("score-button.aria-label"),
                    disabled: !round.read().is_well_defined(),
                    onclick: move |_| {
                        model.write().add_round(round());
                        round.set(Round::default());
                    },
                    {tid!("score-button.text")}
                }
            }
        }
    }
}
