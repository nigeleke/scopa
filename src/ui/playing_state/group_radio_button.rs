use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{Round, Team};

use super::{Group, GroupImage};

#[component]
pub fn GroupRadioButton(
    hint: Option<String>,
    group: Group,
    team: Option<Team>,
    #[props(default = false)] disabled: bool,
) -> Element {
    let mut round = use_context::<Signal<Round>>();

    let id = team.as_ref().map(|t| *t.id());
    let name = team.map(|t| t.name().clone());

    let mut draft = use_signal(move || None);

    use_effect(move || {
        let selection = match group {
            Group::Scopa => unreachable!(),
            Group::CardsCount => round.read().card_count(),
            Group::CoinsCount => round.read().coins_count(),
            Group::Settebello => round.read().settebello(),
            Group::Premiera => round.read().premiera(),
        };

        draft.set(selection);
    });

    let is_checked = *draft.read() == id;
    let settebello_none_disabled =
        group == Group::Settebello && id.is_none() && draft.read().is_some();
    let is_disabled = disabled || settebello_none_disabled;

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_state/group_radio_button.css") }
        label {
            class: "group-radio-button",
            input {
                r#type: "radio",
                name: "{group}",
                value: format!("{group}-{}", id.map_or("none".to_string(), |t| t.to_string())),
                onchange: move |_| match group {
                    Group::Scopa => unreachable!(),
                    Group::CardsCount => round.write().set_card_count(id),
                    Group::CoinsCount => round.write().set_coins_count(id),
                    Group::Settebello => id.into_iter().for_each(|id| round.write().set_settebello(id)),
                    Group::Premiera => round.write().set_premiera(id),
                },
                tabindex: "0",
                aria_label: tid!("score-group-icon.aria-label", group: group.to_string(), teamname: name.map_or("no one".to_string(), |n| n.to_string())),
                disabled: is_disabled,
                checked: is_checked,
            }
            GroupImage {
                hint,
                group: group,
                disabled: is_disabled,
                checked: is_checked,
            }
        }
    }
}
