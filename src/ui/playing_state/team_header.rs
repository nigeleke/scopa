use dioxus::prelude::*;

use crate::domain::Points;
use crate::domain::TeamName;
use crate::ui::glow::Glow;

#[component]
pub fn TeamHeader(name: TeamName, points: Points, is_leader: bool) -> Element {
    rsx! {
        if is_leader {
            span { Glow { "{name}: {points}" } }
        } else {
            span { "{name}: {points}"}
        }
    }
}
