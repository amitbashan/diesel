use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub mod component;
mod theme;
pub mod view;
pub mod widget;

pub use theme::*;
use view::*;

use view::setup::New;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[nest("/event")]
        #[layout(Event)]
            #[route("/:i")]
            EventInstance { i: usize },
        #[end_layout]
    #[end_nest]
    #[route("/")]
    Setup {},
    #[route("/new")]
    New {},
    #[route("/home")]
    Index {},
    #[route("/calendar")]
    Calendar {},
}

pub fn UI(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
