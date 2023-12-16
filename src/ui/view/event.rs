use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::ui::{
    component::{Layout, Navbar},
    Route,
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
            "{i}",
        }
    }
}
