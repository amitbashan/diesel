use std::borrow::Cow;

use chrono::Duration;
use dioxus::prelude::*;

use crate::{hook::use_interval, schedule::Schedule};

use super::WidgetSize;

pub fn UpcomingEventsWidget(cx: &ScopeState, size: WidgetSize) -> Element {
    use_interval(cx, std::time::Duration::new(60, 0));
    let duration = Duration::hours(3);
    let dt = chrono::Local::now().naive_local();
    let schedule = use_shared_state::<Schedule>(cx).unwrap();
    let schedule = schedule.read();
    let events = schedule.events().iter().filter_map(|e| {
        let d = e.datetime() - dt;
        (d <= duration).then_some(e.clone())
    });
    let mut events = events
        .map(|e| {
            let t = e.title();
            let d = e.description();
            let dur = e.datetime() - dt;
            let h = dur.num_hours();
            let m = dur.num_minutes();
            let eta = if h > 0 {
                Cow::Owned(format!("in {h} hours"))
            } else if m == 0 {
                Cow::Borrowed("now")
            } else {
                Cow::Owned(format!("in {m} minutes"))
            };

            render! {
                div {
                    class: "collapse collapse-arrow border border-base-300 join-item",
                    input {
                        r#type: "checkbox"
                    }
                    div {
                        class: "collapse-title text-xl font-medium",
                        "{t}",
                        div {
                            class: "badge badge-primary ml-2",
                            eta.as_ref()
                        }
                    }
                    div {
                        class: "collapse-content text-base",
                        if let Some(d) = d {
                            "{d}"
                        }
                    }
                }
            }
        })
        .peekable();

    render! {
        div {
            class: "flex flex-grow-0 items-start w-80 rounded-box text-xl shadow-xl bg-base-100",
            div {
                class: "join join-vertical w-full m-2",
                if events.peek().is_none() {
                    "No upcoming events"
                }
                events
            }
        }
    }
}
