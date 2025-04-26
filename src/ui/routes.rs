use dioxus::prelude::*;

use super::{
    components::{ScopaFooter, ScopaHeader},
    pages::{Help, Home, NotFound},
};

#[derive(Debug, Clone, Routable, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home,
    #[route("/help/")]
    Help,
    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}

#[component]
fn Layout() -> Element {
    rsx! {
        header { ScopaHeader {} }
        main { Outlet::<Route> {} }
        footer { ScopaFooter {} }
    }
}
