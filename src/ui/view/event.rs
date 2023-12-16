use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    schedule::*,
    ui::{
        component::{Layout, Navbar},
        Route,
    },
};

pub fn Event(cx: Scope) -> Element {
    render! {
        Outlet::<Route> {}
    }
}

#[component]
pub fn EventInstance(cx: Scope, i: usize) -> Element {
    render! {
        Layout {
            navbar: render! { Navbar {} },
            EventDisplay { i: *i },
        }
    }
}

#[component]
pub fn EventDisplay(cx: Scope, i: usize) -> Element {
    let schedule = use_shared_state::<Schedule>(cx).unwrap();
    let schedule = schedule.read();
    let event = schedule.get_event(*i);
    let title = event.title();
    let description = event.description().map(|d| d.as_str()).unwrap_or_default();
    let description = if description.is_empty() {
        render! {
            p {
                class: "italic placeholder-base-content",
                "No description provided."
            }
        }
    } else {
        render! {
            p { description }
        }
    };

    render! {
        div {
            class: "flex-1 rounded p-6",
            article {
                class: "prose",
                h3 {
                    class: "font-bold",
                    "{title}",
                }
                description,
            }
        }
    }
}
