use std::borrow::Cow;

use chrono::{prelude::*, Duration};
use dioxus::prelude::*;

use crate::{
    hook::use_interval, lang::Context, schedule::Schedule, ui::component::EventTitleButton,
};

use super::{WidgetSize, WidgetStates};

pub fn UpcomingEventsWidget<'a>(
    cx: &'a ScopeState,
    _: WidgetSize,
    _: &'a UseSharedState<WidgetStates>,
) -> Element<'a> {
    use_interval(cx, std::time::Duration::from_secs(60));
    let datetime = Local::now().naive_local();
    let schedule = use_shared_state::<Schedule>(cx)?;
    let schedule = schedule.read();
    let mut events: Vec<_> = schedule
        .query_with_context(Context {
            date: datetime.date(),
        })
        .filter(|(_, event)| event.time_pair().as_ref().get().0 .0 > datetime.time())
        .collect();

    if events.is_empty() {
        render! {
            div {
                class: "flex flex-1 h-full rounded-lg text-sm items-center justify-center shadow-xl bg-base-100 font-mono",
                "No events for today."
            }
        }
    } else {
        events.sort_by_key(|(_, event)| event.as_ref().time_pair().get().0 .0);
        let next_event_time = events[0].1.as_ref().time_pair().as_ref().get().0 .0;
        let next_event_eta = next_event_time - datetime.time();
        let hours = next_event_eta.num_hours();
        let minutes = (next_event_eta - Duration::hours(hours)).num_minutes();
        let eta = match (hours, minutes) {
            (0, 0) => Cow::Borrowed("now"),
            (0, minutes) => Cow::Owned(format!("in {minutes} minutes")),
            (hours, 0) => Cow::Owned(format!("in {hours} hours")),
            (hours, minutes) => Cow::Owned(format!("in {hours} hours and {minutes} minutes")),
        };
        let event_buttons = events.iter().map(|(i, _)| {
            render! {
                EventTitleButton { i: *i }
            }
        });
        let displayed_events = render! {
            div {
                class: "join join-vertical rounded-none h-0 flex flex-col flex-wrap grow overflow-hidden",
                event_buttons
            }
        };

        render! {
            div {
                class: "flex flex-1 h-full rounded-lg shadow-xl bg-base-100",
                div {
                    class: "flex flex-1 flex-col rounded-lg m-2",
                    p {
                        class: "text-xs font-bold m-1",
                        format!("Next event: {eta}.")
                    }
                    displayed_events
                }
            }
        }
    }
}
