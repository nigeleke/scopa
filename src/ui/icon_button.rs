use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Icon {
    Home,
    Help,
    Menu,
    English,
    Italian,
    Reset,
    AddTeam,
    RemoveTeam,
    Start,
    Undo,
    Fullscreen,
}

#[component]
pub fn IconButton(
    icon: Icon,
    #[props(default)] class: Option<String>,
    on_click: EventHandler<()>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    let asset = match icon {
        Icon::Home => asset!("/assets/images/icon-home-page.png"),
        Icon::Help => asset!("/assets/images/icon-help.png"),
        Icon::Menu => asset!("/assets/images/icon-hamburger.png"),
        Icon::English => asset!("/assets/images/icon-flag-gb.png"),
        Icon::Italian => asset!("/assets/images/icon-flag-it.png"),
        Icon::Reset => asset!("/assets/images/icon-reset.png"),
        Icon::AddTeam => asset!("/assets/images/icon-add.png"),
        Icon::RemoveTeam => asset!("/assets/images/icon-cancel.png"),
        Icon::Start => asset!("/assets/images/icon-play.png"),
        Icon::Undo => asset!("/assets/images/icon-return.png"),
        Icon::Fullscreen => asset!("/assets/images/icon-fit-to-width.png"),
    };

    let class = match &class {
        Some(c) => format!("icon-button {c}"),
        None => "icon-button".to_string(),
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/icon_button.css") }
        button {
            class: "{class}",
            onclick: move |_| on_click.call(()),
            ..attributes,
            img { src: asset }
        }
    }
}
