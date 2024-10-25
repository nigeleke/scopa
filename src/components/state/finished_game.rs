use crate::components::prelude::*;
use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn FinishedGame(
    state: FinishedState,
    onchange: EventHandler<GameState>,
) -> Element {
    let winner = state.winner();

    let mut teams = Vec::from(state.teams());
    teams.sort_by_key(|b| std::cmp::Reverse(state.points(b.id())));

    let team_points = teams.into_iter()
        .map(|team: Team| rsx!{ TeamPoints { state: state.clone(), team: team } })
        .collect::<Vec<_>>();

    let mut retain_players = use_signal(|| true);

    let update_retain_players = move |value: String| {
        retain_players.set(str::parse::<bool>(&value).unwrap());
    };

    let mut retain_target = use_signal(|| true);

    let update_retain_target = move |value: String| {
        retain_target.set(str::parse::<bool>(&value).unwrap());
    };

    let start_new_game = move || {
        let mut new_game = Game::default();

        if retain_players() {
            let teams = Vec::from(state.teams());
            teams.iter().for_each(|t| {
                let team = Team::new(&t.name());
                let _ = new_game.add_team(team); } 
            );
        }

        if retain_target() {
            new_game.set_target(state.target());
        }

        if retain_players() && retain_target() {
            onchange.call(new_game.start().unwrap());
        } else {
            onchange.call(GameState::Starting(new_game));
        }
    };

    rsx! {
        div {
            class: "finished-game-container",
            div {
                class: "finished-game-winner-text",
                Glow { "Winner - " { winner } }
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
                        "Start again"
                    }
                }
                div {
                    class: "finished-game-retain-group",
                    div {
                        class: "finished-game-retain-checkbox",
                        Input {
                            typ: "checkbox",
                            value: retain_players(),
                            checked: retain_players(),
                            on_input: update_retain_players,
                            aria_label: "Check to use same players in new game",
                        }
                        { " Same players" }    
                    }
                    div {
                        class: "finished-game-retain-checkbox",
                        Input {
                            typ: "checkbox",
                            value: retain_target(),
                            checked: retain_target(),
                            on_input: update_retain_target,
                            aria_label: "Check to use same target in new game",
                        }
                        { " Same target" }    
                    }
                }
            }
        }
    }
}

#[component]
fn TeamPoints(
    state: FinishedState,
    team: Team,
) -> Element {
    let name = team.name();
    let points = state.points(team.id());
    rsx ! {
        div {
            class: "finished-game-team-score",
            span { {name} }
            span { {points.to_string()} }
        }
    }
}