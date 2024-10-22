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
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {

    let update_team_name = move |event: Event<FormData>| {
        let new_team_name = TeamName::try_from(event.value()).unwrap();
        team_name.set(new_team_name);
    };

    let commit_team_edit = move |event: KeyboardEvent| {
        if event.key() == Key::Enter {
            oncommit.call(team_name());        
        }        
    };

    rsx! {
        input {
            value: team_name().to_string(),
            autofocus: true,
            oninput: update_team_name,
            onkeydown: commit_team_edit, 
            ..attributes,
        }
    }
}
