use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod component;
mod view;
pub mod widget;

use view::*;

use crate::{
    schedule::Schedule,
    ui::widget::{UpcomingEventsWidgetState, WidgetManagerState},
};

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
    Index {},
    #[route("/calendar")]
    Calendar {},
}

pub fn UI(cx: Scope) -> Element {
    use_shared_state_provider(cx, || UpcomingEventsWidgetState::default());
    use_shared_state_provider(cx, || WidgetManagerState::default());
    use_shared_state_provider(cx, || Schedule::default());

    render! {
        Router::<Route> {}
    }
}
