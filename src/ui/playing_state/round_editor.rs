use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::application::Model;
use crate::domain::Team;

use super::{Group, GroupRadioButton, RoundNumber, ScopaIcon, ScopaScore, TeamHeader};

#[component]
pub fn RoundEditor() -> Element {
    let model = use_context::<Signal<Model>>();

    let first_active_team_id = *model
        .read()
        .active_teams()
        .next()
        .expect("playing_state should have at least 1 active team")
        .id();

    let leading_teams_score = model.read().score(
        *model
            .read()
            .leading_teams()
            .next()
            .expect("playing_state should have at least 1 leading team")
            .id(),
    );

    let none_column_components = [
        rsx! { RoundNumber { } },
        rsx! { ScopaIcon { hint: tid!("scopa-icon.hint") } },
        rsx! { GroupRadioButton { hint: tid!("cards-count-icon.hint"), group: Group::CardsCount, team: None } },
        rsx! { GroupRadioButton { hint: tid!("coins-count-icon.hint"), group: Group::CoinsCount, team: None } },
        rsx! { GroupRadioButton { hint: tid!("settebello-icon.hint"), group: Group::Settebello, team: None } },
        rsx! { GroupRadioButton { hint: tid!("premiera-icon.hint"), group: Group::Premiera, team: None } },
    ];

    let team_column_components = move |team: &Team| {
        let id = *team.id();
        let name = team.name();
        let points = model.read().score(id);
        let is_leader = points == leading_teams_score;
        let is_not_playing = !team.is_playing();
        [
            rsx! { TeamHeader { name: name.clone(), points: points, is_leader: is_leader } },
            rsx! { ScopaScore { team: team.clone(), autofocus: id == first_active_team_id, disabled: is_not_playing } },
            rsx! { GroupRadioButton { group: Group::CardsCount, team: Some(team.clone()), disabled: is_not_playing } },
            rsx! { GroupRadioButton { group: Group::CoinsCount, team: Some(team.clone()), disabled: is_not_playing } },
            rsx! { GroupRadioButton { group: Group::Settebello, team: Some(team.clone()), disabled: is_not_playing } },
            rsx! { GroupRadioButton { group: Group::Premiera, team: Some(team.clone()), disabled: is_not_playing } },
        ]
    };

    let some_column_components = model
        .read()
        .teams()
        .map(team_column_components)
        .collect::<Vec<_>>();
    let columns_count = some_column_components.len() + 1;

    let cells = (0..columns_count).flat_map(|j| {
        let column = match j {
            0 => &none_column_components,
            _ => &some_column_components[j - 1],
        };
        column.iter().cloned()
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/playing_state/round_editor.css") }
        div {
            class: "round-editor",
            {cells}
        }
    }
}
