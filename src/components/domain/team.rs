use super::team_name::TeamNameEditor;

use crate::domain::prelude::Team;

use dioxus::prelude::*;

#[component]
pub fn TeamEditor(
    team: Team,
    onchange: EventHandler<Team>,
) -> Element {
    let team = use_signal(move || team);

    let mut team_name = use_signal(move || team.read().name());
    let update_team_name = move |new_name| team_name.set(new_name);
    
    let commit_team_name = move |_| {
        let mut team = team();
        team.set_name(team_name());
        onchange.call(team);
    };

    rsx! {
        TeamNameEditor {
            value: team_name(),
            onchange: update_team_name,
            oncommit: commit_team_name,
        }
    }
}