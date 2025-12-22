use dioxus::prelude::*;
use dioxus_i18n::tid;
use dioxus_sdk::storage::{LocalStorage, use_storage};

use crate::{
    domain::*,
    ui::{
        components::{
            HelpIcon,
            button::Button,
            target::TargetEditor,
            team_name::{TeamNameEditor, TeamNameView},
        },
        consts::STORAGE_TARGET,
        pages::Page,
        state::State,
    },
};

#[component]
pub fn StartingGame(game: Game<Starting>, onchange: EventHandler<State>) -> Element {
    let mut game = use_signal(|| game);
    let mut default_target = use_storage::<LocalStorage, _>(STORAGE_TARGET.into(), Target::default);

    let target = game.read().target();
    let update_target = move |new_target| {
        game.write().set_target(new_target);
        default_target.set(new_target);
    };

    let teams = Teams::from(game.read().teams().clone());
    let add_team = move |team| {
        game.write().add_team(team);
    };
    let remove_team = move |id| {
        game.write().remove_team(id)?;
        Ok(())
    };

    let start_game = move |_| {
        let game = game().start()?;
        onchange.call(State::from(game));
        Ok(())
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/pages/starting_game.css") }
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
                teams: teams,
                onremove: remove_team,
            }
            HelpAction {}
        }
    }
}

#[component]
fn AddTeam(onadd: EventHandler<Team>) -> Element {
    let mut team_name = use_signal(TeamName::default);

    let mut add_team = move |value: TeamName| {
        let team = Team::from(value.to_string().as_str());
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
                td { TeamNameView { value: team.name().clone() } }
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

#[component]
fn HelpAction() -> Element {
    let mut page = use_context::<Signal<Page>>();

    let on_help = move |_| {
        page.set(Page::Help);
    };

    rsx! {
        HelpIcon { on_click: on_help }
    }
}
