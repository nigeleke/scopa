use dioxus::prelude::*;

use crate::ui::playing_state::group::Group;
use crate::ui::playing_state::group_image::GroupImage;

#[component]
pub fn ScopaIcon(hint: Option<String>) -> Element {
    rsx! {
        GroupImage { hint, group: Group::Scopa, disabled: false, checked: true }
    }
}
