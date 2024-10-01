use super::target::TargetEditor;
use super::team_name::{TeamNameEditor, TeamNameView};

use crate::domain::*;
use crate::types::*;

use dioxus::prelude::*;

#[component]
pub fn StartingGame() -> Element {
    rsx! {
        AddTeamInput {}
        StartAction {}
        TeamRows {}
    }
}

#[component]
fn AddTeamInput() -> Element {
    let mut team_name = use_signal(move || TeamName::default());

    let mut context = use_context::<Signal<Game>>();

    let add_team = move |value: TeamName| {
        let mut game = context();
        let _ = game.add_team(&value.to_string());
        context.set(game);
        team_name.set(TeamName::default());
    };

    rsx! {
        TeamNameEditor {
            value: team_name,
            autofocus: true,
            oncommit: add_team,
        }
    }
}

#[component]
fn TeamRows() -> Element {
    let mut context = use_context::<Signal<Game>>();
    let teams = use_memo(move || context().teams());

    let remove_team = move |team: &Team| {
        let id = team.id();
        move |_| {
            let mut game = context();
            let _ = game.remove_team(&id);
            context.set(game);
        }
    };

    let team_row = move |team: &Team| {
        rsx! {
            tr { 
                td { button { onclick: remove_team(&team),  "-"  } }
                td { TeamNameView { value: team.name() } }
            }
        }    
    };

    rsx! {
        table {
            hidden: teams().is_empty(),
            { teams().iter().map(team_row) }
        }
    }
}

#[component]
fn StartAction() -> Element {
    let mut context = use_context::<Signal<Game>>();

    let target = use_signal(move || 16.into());

    let can_start = context().can_start();

    let start = move |_| {
        let game = context().start(target()).unwrap();
        context.set(game);
    };

    rsx! {
        div {
            TargetEditor {
                value: target,
            }
        }
        button {
            disabled: !can_start,
            onclick: start,
            "Start"
        }
    }
   
}