use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::application::Model;
use crate::domain::{Target, TargetError, Team, TeamId, TeamName};
use crate::ui::icon_button::IconButton;

#[component]
pub fn SetupState() -> Element {
    let mut model = use_context::<Signal<Model>>();

    let mut valid_target = use_signal(|| true);
    let mut team_name = use_signal(TeamName::default);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/setup_state.css") }
        div {
            class: "setup-state",

            TargetInput {
                value: model.read().target(),
                on_value_changed: move |t| model.write().set_target(t),
                on_validiation_status: move |v: Result<_, _>| valid_target.set(v.is_ok())
            }

            TeamNameInput {
                value: team_name(),
                on_value_changed: move |n| team_name.set(n),
                on_submit: move |_| {
                    model.write().add_team(team_name());
                    team_name.set(TeamName::default());
                },
                can_submit: model.read().can_add_team(&team_name()),
            }

            TeamsList {
                teams: model.read().teams().cloned().collect(),
                on_remove: move |id| model.write().remove_team(id),
            }

            StartButton {
                on_start: move |_| model.write().start(),
                can_start: model.read().can_start() && *valid_target.read()
            }

            HelpButton {
                on_help: move |_| model.write().show_help(),
            }
        }
    }
}

#[component]
fn TargetInput(
    value: Target,
    on_value_changed: EventHandler<Target>,
    on_validiation_status: EventHandler<Result<Target, TargetError>>,
) -> Element {
    use std::str::FromStr;

    let mut value = use_signal(|| value.to_string());
    let mut validated_value = use_signal(|| Target::from_str(value.read().as_str()));

    use_effect(move || {
        if let Ok(value) = validated_value() {
            on_value_changed.call(value);
        }
        on_validiation_status.call(validated_value())
    });

    rsx! {
        div {
            class: "setup-state__target-input",

            span { {tid!("target-input.label")} }

            input {
                name: "target",
                placeholder: tid!("target-input.placeholder"),
                type: "number",
                min: "1",
                step: "5",
                required: "true",
                value,
                oninput: move |e| {
                    value.set(e.value());
                    validated_value.set(Target::from_str(&e.value()));
                },
            }
        }
    }
}

#[component]
fn TeamNameInput(
    value: TeamName,
    on_value_changed: EventHandler<TeamName>,
    on_submit: EventHandler<()>,
    can_submit: bool,
) -> Element {
    rsx! {
        div {
            class: "setup-state__team-name-input",

            input {
                name: "team-name",
                placeholder: tid!("team-name-input.placeholder"),
                value: value.to_string(),
                oninput: move |e| on_value_changed.call(TeamName::new(e.value())),
            }

            IconButton {
                icon: "+",
                on_click: move |_| on_submit.call(()),
                title: tid!("add-team-button.hint"),
                aria_label: tid!("add-team-button.aria-label"),
                disabled: !can_submit,
            }
        }
    }
}

#[component]
fn TeamsList(teams: Vec<Team>, on_remove: EventHandler<TeamId>) -> Element {
    rsx! {
        ul {
            class: "setup-state__teams-list",
            for team in teams {
                TeamRow {
                    team: team.clone(),
                    on_remove,
                }
            }
        }
    }
}

#[component]
fn TeamRow(team: Team, on_remove: EventHandler<TeamId>) -> Element {
    let team_id = *team.id();
    let team_name = team.name();

    rsx! {
        li {
            class: "setup-state__team-row",
            IconButton {
                icon: "-",
                on_click: move |_| on_remove.call(team_id),
                title: tid!("remove-team-button.hint"),
                aria_label: tid!("remove-team-button.aria-label", team: team_name.to_string()),
            }
            span { "{team_name}" }
        }
    }
}

#[component]
fn StartButton(on_start: EventHandler<()>, can_start: bool) -> Element {
    rsx! {
        button {
            aria_label: tid!("start-button.aria-label"),
            disabled: !can_start,
            onclick: move |_| on_start(()),
            {tid!("start-button.text")} }
    }
}

#[component]
fn HelpButton(on_help: EventHandler<()>) -> Element {
    rsx! {
        button {
            aria_label: tid!("help-button.aria-label"),
            onclick: move |_| on_help(()),
            {tid!("help-button.text")}
        }
    }
}
