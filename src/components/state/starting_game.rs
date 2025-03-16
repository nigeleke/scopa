use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_i18n::tid;

#[component]
pub fn StartingGame(state: StartingState, onchange: EventHandler<GameState>) -> Element {
    let mut state = use_signal(move || state);

    let target = state.read().target();
    let update_target = move |new_target| {
        state.write().set_target(new_target);
    };

    let teams = Vec::from(state.read().teams());
    let add_team = move |team| {
        state.write().add_team(team).unwrap();
    };
    let remove_team = move |id| {
        state.write().remove_team(id).unwrap();
    };

    let start_game = move |_| {
        let new_game = state.read().start().unwrap();
        onchange.call(new_game);
    };

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/state/starting_game.css") }
        div {
            class: "starting-game",
            TargetEditor {
                value: target,
                onchange: update_target,
            }
            AddTeam {
                onadd: add_team,
            }
            StartAction {
                can_start: state.read().can_start(),
                onstart: start_game,
            }
            TeamRows {
                teams: teams,
                onremove: remove_team,
            }
    }
    }
}

#[component]
fn AddTeam(onadd: EventHandler<Team>) -> Element {
    let mut team_name = use_signal(TeamName::default);

    let mut add_team = move |value: TeamName| {
        let team = Team::new(&value);
        onadd.call(team);
        team_name.set(TeamName::default());
    };

    let add_team_button_action = move |_| add_team(team_name());

    rsx! {
        span {
            TeamNameEditor {
                team_name: team_name,
                autofocus: true,
                oncommit: add_team,
                placeholder: tid!("team-name-editor.placeholder"),
            }
            " "
            Button { on_click: add_team_button_action, " + " }
        }
    }
}

#[component]
fn TeamRows(teams: Vec<Team>, onremove: EventHandler<TeamId>) -> Element {
    let remove_team = move |team: &Team| {
        let id = team.id();
        move |_event| {
            onremove.call(id);
        }
    };

    let team_row = move |team: &Team| {
        rsx! {
            tr {
                td { Button { on_click: remove_team(team),  "-"  } }
                td { TeamNameView { value: team.name() } }
            }
        }
    };

    rsx! {
        table {
            hidden: teams.is_empty(),
            { teams.iter().map(team_row) }
        }
    }
}

#[component]
fn StartAction(can_start: bool, onstart: EventHandler<()>) -> Element {
    let start = move |_| {
        onstart.call(());
    };

    rsx! {
        Button {
            disabled: !can_start,
            on_click: start,
            {tid!("start-button.text")}
        }
    }
}
