#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use crate::{
    configuration::ConfigurationPath,
    schedule::Schedule,
    ui::{
        widget::{WidgetManagerState, WidgetStates},
        Theme,
    },
};

mod configuration;
mod hook;
mod ql;
mod schedule;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");
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
    use_shared_state_provider(cx, ConfigurationPath::default);

    let theme = use_shared_state::<Theme>(cx)?.with(|t| t.0.to_string());

    render! {
        style { include_str!("./css/extra.css") }
        style { include_str!("./css/tailwind.css") }
        div {
            "data-theme": theme,
            ui::UI {}
        }
    }
}
