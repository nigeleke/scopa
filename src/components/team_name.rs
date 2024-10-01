use crate::types::TeamName;

use dioxus::prelude::*;

#[component]
pub fn TeamNameView(
    value: TeamName,
) -> Element {

    rsx! { 
        { value.to_string() } 
    }
}

#[component]
pub fn TeamNameEditor(
    value: Signal<TeamName>,
    oncommit: EventHandler<TeamName>,
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {

    let update_team_name = move |event: Event<FormData>| {
        value.set(TeamName::from(event.value().as_str()));
    };

    let commit_team_edit = move |event: KeyboardEvent| {
        if event.key() == Key::Enter {
            oncommit.call(value());        
        }        
    };

    rsx! {
        input {
            value: value().to_string(),
            oninput: update_team_name,
            onkeydown: commit_team_edit, 
            ..attributes,
        }
    }
}
