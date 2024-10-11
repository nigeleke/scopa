use crate::types::TeamName;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Value};

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
    value: TeamName,
    onchange: EventHandler<TeamName>,
    oncommit: EventHandler<TeamName>,
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {

    let mut team_name = use_signal(move || TeamName::default());

    let update_team_name = move |event: Event<FormData>| {
        let new_team_name = TeamName::try_from(event.value()).unwrap();
        team_name.set(new_team_name);
    };

    let commit_team_edit = move |event: KeyboardEvent| {
        if event.key() == Key::Enter {
            oncommit.call(team_name());        
            team_name.set(TeamName::default());
        }        
    };

    rsx! {
        input {
            value: team_name().to_string(),
            placeholder: "Enter a team name",
            autofocus: true,
            oninput: update_team_name,
            onkeydown: commit_team_edit, 
            ..attributes,
        }
    }
}
