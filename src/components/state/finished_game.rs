use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_i18n::t;

#[component]
pub fn FinishedGame(state: FinishedState, onchange: EventHandler<GameState>) -> Element {
    let winner = state.winner();

    let mut teams = Vec::from(state.teams());
    teams.sort_by_key(|b| std::cmp::Reverse(state.points(b.id())));

    let team_points = teams
        .into_iter()
        .map(|team: Team| rsx! { TeamPoints { state: state.clone(), team: team } })
        .collect::<Vec<_>>();

    let mut retain_settings = use_signal(|| true);

    let update_retain_settings = move |value: String| {
        retain_settings.set(str::parse::<bool>(&value).unwrap());
    };

    let start_new_game = move || {
        let mut new_game = Game::default();

        if retain_settings() {
            let teams = Vec::from(state.teams());
            teams.iter().for_each(|t| {
                let team = Team::new(&t.name());
                let _ = new_game.add_team(team);
            });
        }

        if retain_settings() {
            onchange.call(new_game.start().unwrap());
        } else {
            onchange.call(GameState::Starting(new_game));
        }
    };

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/state/finished_game.css") }
        div {
            class: "finished-game-container",
            div {
                class: "finished-game-winner-text",
                Glow { {t!("winner-view", teamname: winner.to_string())} }
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
                        {t!("start-new-game-button-text")}
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
                            aria_label: t!("start-new-game-settings-aria-label"),
                        }
                        { t!("start-new-game-settings-text") }
                    }
                }
            }
        }
    }
}

#[component]
fn TeamPoints(state: FinishedState, team: Team) -> Element {
    let name = team.name();
    let points = state.points(team.id());
    rsx! {
        div {
            class: "finished-game-team-score",
            span { {name} }
            span { {points.to_string()} }
        }
    }
}
