#![allow(non_snake_case)]

use std::{io, path::PathBuf};

use configuration::HomeConfiguration;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::prelude::*;
use log::info;

use crate::ui::{component::ErrorModal, Theme};

mod configuration;
mod hook;
mod ql;
mod schedule;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    HomeConfiguration::default()
        .create_if_does_not_exist()
        .expect("failed to create home configuration");
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new().with_title("Diesel")),
    );
}

fn App(cx: Scope) -> Element {
    let home_configuration = use_state(cx, HomeConfiguration::read);

    match home_configuration.get() {
        Ok(Some(cfg)) => render! {
            "cfg"
        },
        Ok(_) => render! {
            ErrorModal {
                p { "Failed to deserialize" }
            }
        },
        Err(e) => render! {
            render! {
                ErrorModal {
                    p { "{e}" }
                }
            }
        },
    }
}
