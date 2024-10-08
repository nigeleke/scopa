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
    value: TeamName,
    onchange: EventHandler<TeamName>,
    oncommit: EventHandler<TeamName>,
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {

    let mut team_name = use_signal(move || value);

    let update_team_name = move |event: Event<FormData>| {
        team_name.set(TeamName::from(event.value().as_str()));
    };

    use_effect(move || { onchange.call(team_name()); });

    let commit_team_edit = move |event: KeyboardEvent| {
        if event.key() == Key::Enter {
            oncommit.call(team_name());        
        }        
    };

    rsx! {
        input {
            value: team_name().to_string(),
            oninput: update_team_name,
            onkeydown: commit_team_edit, 
            ..attributes,
        }
    }
}
