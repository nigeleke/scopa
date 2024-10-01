#![feature(variant_count)]

mod components;
mod domain;
mod pages;
mod types;

use crate::pages::home::Home;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("Cannot start logging");
    launch(app);
}

fn app() -> Element {
    rsx! {
        Home {}
    }
}
