use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{Points, Round, Team};

use super::PointsEditor;

#[component]
pub fn ScopaScore(team: Team, autofocus: bool, disabled: bool) -> Element {
    let mut round = use_context::<Signal<Round>>();

    let id = *team.id();
    let name = team.name();

    let mut draft = use_signal(Points::zero);

    use_effect(move || draft.set(round.read().scopas(id)));

    rsx! {
        PointsEditor {
            value: draft(),
            on_change: move |points| round.write().set_scopas(id, points),
            autofocus: autofocus,
            disabled: disabled,
            aria_label: tid!("score-scopa-editor.aria-label", teamname: name.to_string()),
        }
    }
}
