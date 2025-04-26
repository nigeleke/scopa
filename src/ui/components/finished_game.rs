use dioxus::prelude::*;
use dioxus_i18n::tid;
use dioxus_sdk::storage::{LocalStorage, use_storage};

use super::{button::Button, glow::Glow, input::Input};
use crate::{
    domain::*,
    ui::{consts::*, state::State},
};

#[component]
pub fn FinishedGame(game: Game<Finished>, onchange: EventHandler<State>) -> Element {
    let game = use_signal(|| game);
    let default_target = use_storage::<LocalStorage, _>(STORAGE_TARGET.into(), Target::default);

    let winner = game.read().winner_name()?;

    let mut teams = Teams::from(game.read().teams().clone());
    teams.sort_by_key(|b| std::cmp::Reverse(game.read().points(b.id()).expect("valid points")));

    let team_points = teams
        .into_iter()
        .map(|team: Team| rsx! { TeamPoints { game: game(), team: team } })
        .collect::<Vec<_>>();

    let mut retain_settings = use_signal(|| true);

    let update_retain_settings = move |value: String| {
        retain_settings.set(str::parse::<bool>(&value).expect("true or false"));
    };

    let start_new_game = move || {
        let mut new_game = Game::from(default_target());

        if retain_settings() {
            let teams = game.read().teams().clone();
            teams.iter().for_each(|t| {
                let team = Team::from(t.name().to_string().as_str());
                new_game.add_team(team);
            });
        }

        if retain_settings() {
            let game = new_game.start()?;
            onchange.call(State::from(game));
        } else {
            onchange.call(State::from(new_game));
        }

        Ok(())
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/state/finished_game.css") }
        div {
            class: "finished-game-container",
            div {
                class: "finished-game-winner-text",
                Glow { {tid!("winner-view.text", teamname: winner.to_string())} }
            }
            div {
                class: "finished-game-team-scores",
                for team_points in team_points {
                    {team_points}
                }
            }
            div {
                class: "finished-game-controls",
                div {
                    class: "finished-game-start-game-button",
                    Button {
                        on_click: start_new_game,
                        {tid!("start-new-game-button.text")}
                    }
                }
                div {
                    class: "finished-game-retain-group",
                    div {
                        class: "finished-game-retain-checkbox",
                        Input {
                            typ: "checkbox",
                            value: retain_settings(),
                            checked: retain_settings(),
                            on_input: update_retain_settings,
                            aria_label: tid!("start-new-game-settings.aria-label"),
                        }
                        { tid!("start-new-game-settings.text") }
                    }
                }
            }
        }
    }
}

#[component]
fn TeamPoints(game: Game<Finished>, team: Team) -> Element {
    let name = team.name();
    let points = game.points(team.id())?;
    rsx! {
        div {
            class: "finished-game-team-score",
            span { {name.to_string()} }
            span { {points.to_string()} }
        }
    }
}
