use crate::components::prelude::*;
use crate::domain::prelude::*;
use crate::types::*;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn StartingGame(
    game: Game<StartingState>,
    onchange: EventHandler<GameState>,
) -> Element {

    let mut game = use_signal(move || game.clone());

    use_effect(move || {
        onchange.call(GameState::Starting(game()));
        info!("Game changed: {:?}", game())
    });

    let target = game.read().target();
    let update_target = move |new_target| {
        game.write().set_target(new_target);
    };

    let teams = Vec::from(game.read().teams());
    let add_team = move |team| { 
        game.write().add_team(team).unwrap();
    };
    let remove_team = move |id| {
        game.write().remove_team(id).unwrap(); 
    };

    let start_game = move |_| {
        let new_game = game().start().unwrap();
        onchange.call(new_game);
    };

    rsx! {
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
                can_start: game.read().can_start(),
                onstart: start_game,
            }
            TeamRows {
                teams: Vec::from(teams),
                onremove: remove_team,
            }
    }
    }
}

#[component]
fn AddTeam(
    onadd: EventHandler<Team>,
) -> Element {
    let mut team_name = use_signal(move || TeamName::default());

    let update_team_name = move |new_name| {
        team_name.set(new_name)
    };

    let add_team = move |value: TeamName| {
        let team = Team::new(&value);
        onadd.call(team);
    };

    rsx! {
        TeamNameEditor {
            value: team_name(),
            autofocus: true,
            onchange: update_team_name,
            oncommit: add_team,
        }
    }
}

#[component]
fn TeamRows(
    teams: Vec<Team>,
    onremove: EventHandler<TeamId>,
) -> Element {

    let remove_team = move |team: &Team| {
        let id = team.id();
        move |_event| {
            onremove.call(id);
        }
    };

    let team_row = move |team: &Team| {
        rsx! {
            tr { 
                td { button { onclick: remove_team(team),  "-"  } }
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
fn StartAction(
    can_start: bool,
    onstart: EventHandler<()>,
) -> Element {

    let start = move |_| { onstart.call(()); };

    rsx! {
        button {
            disabled: !can_start,
            onclick: start,
            "Start"
        }
    }
   
}