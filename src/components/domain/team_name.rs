use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn TeamNameView(
    value: TeamName,
) -> Element {
    rsx! { 
        span { { value.to_string() } }
    }
}

#[component]
pub fn TeamNameEditor(
    team_name: Signal<TeamName>,
    oncommit: EventHandler<TeamName>,
    autofocus: bool,
    placeholder: String,
) -> Element {

    let update_team_name = move |value| {
        let new_team_name = TeamName::try_from(value).unwrap();
        team_name.set(new_team_name);
    };

    let commit_team_edit = move |_| { oncommit.call(team_name()); };

    rsx! {
        label {
            Input {
                typ: "text",
                value: team_name().to_string(),
                on_input: update_team_name,
                on_commit: commit_team_edit, 
                autofocus,
                placeholder,
                aria_label: "Enter team name",
            }
        }
    }
}
