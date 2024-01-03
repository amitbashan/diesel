#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use crate::{
    configuration::ConfigurationManager,
    hook::use_save,
    schedule::Schedule,
    ui::{
        widget::{WidgetManagerState, WidgetStates},
        Theme,
    },
};

mod configuration;
mod hook;
mod lang;
mod schedule;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new().with_title("Diesel")),
    );
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, Theme::default);
    use_shared_state_provider(cx, WidgetManagerState::default);
    use_shared_state_provider(cx, WidgetStates::default);
    use_shared_state_provider(cx, Schedule::default);
    use_shared_state_provider(cx, {
        let theme = use_shared_state::<Theme>(cx)?.clone();
        let widget_manager_state = use_shared_state::<WidgetManagerState>(cx)?.clone();
        let widget_states = use_shared_state::<WidgetStates>(cx)?.clone();
        let schedule = use_shared_state::<Schedule>(cx)?.clone();
        || ConfigurationManager {
            path: None,
            theme,
            widget_manager_state,
            widget_states,
            schedule,
        }
    });

    let save = use_save(cx);
    use_on_destroy(cx, save);
    let theme = use_shared_state::<Theme>(cx)?.with(|t| t.0.to_string());

    render! {
        style { include_str!("./css/tailwind.css") }
        div {
            "data-theme": theme,
            ui::UI {}
        }
    }
}
