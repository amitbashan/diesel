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
    use_shared_state_provider(cx, || {
        Schedule::new(vec![
            std::rc::Rc::new(crate::schedule::BasicEvent {
                title: std::rc::Rc::new("Test 1".to_string()),
                description: std::rc::Rc::new("Test 1 description".to_string()),
                date: chrono::Local::now().naive_local(),
                duration: chrono::Duration::hours(10),
            }),
            std::rc::Rc::new(crate::schedule::BasicEvent {
                title: std::rc::Rc::new("Test 2".to_string()),
                description: std::rc::Rc::new("Test 2 description".to_string()),
                date: chrono::Local::now().naive_local(),
                duration: chrono::Duration::hours(10),
            }),
            std::rc::Rc::new(crate::schedule::BasicEvent {
                title: std::rc::Rc::new("Test 3".to_string()),
                description: std::rc::Rc::new("Test 3 description".to_string()),
                date: chrono::Local::now().naive_local(),
                duration: chrono::Duration::hours(10),
            }),
            std::rc::Rc::new(crate::schedule::BasicEvent {
                title: std::rc::Rc::new("Test 4".to_string()),
                description: std::rc::Rc::new("Test 4 description".to_string()),
                date: chrono::Local::now().naive_local(),
                duration: chrono::Duration::hours(10),
            }),
        ])
    });

    render! {
        Router::<Route> {}
    }
}
