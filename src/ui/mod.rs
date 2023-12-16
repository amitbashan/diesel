use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod component;
mod view;
pub mod widget;

use view::{Calendar, Index};

use crate::{schedule::Schedule, ui::widget::WidgetManagerState};

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Index {},
    #[route("/calendar")]
    Calendar {},
}

pub fn UI(cx: Scope) -> Element {
    use_shared_state_provider(cx, || WidgetManagerState::default());
    use_shared_state_provider(cx, || Schedule::default());

    render! {
        Router::<Route> {}
    }
}
