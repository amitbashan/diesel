use dioxus::prelude::*;

use crate::configuration::{Configuration, ConfigurationManager};

pub fn use_save(cx: &ScopeState) -> impl Fn() {
    let cfg_manager = use_shared_state::<ConfigurationManager>(cx)
        .expect("configuration manager not initialized")
        .clone();

    move || {
        Configuration::try_save(&cfg_manager).expect("failed to save configuration");
    }
}
