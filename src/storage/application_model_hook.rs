use dioxus::prelude::*;
use dioxus_sdk::storage::{LocalStorage, use_storage};

use crate::application::Model;

pub const STORAGE_KEY: &str = "scopa-model";

pub fn use_application_model() -> Signal<Model> {
    use_storage::<LocalStorage, _>(STORAGE_KEY.into(), Model::default)
}
