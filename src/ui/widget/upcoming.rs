use chrono::prelude::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    hook::use_interval,
    ql::grammar,
    schedule::{event::Time, Schedule},
    ui::component::EventTitleButton,
};

use super::WidgetSize;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UpcomingEventsWidgetState {
    within: Time,
}

impl Default for UpcomingEventsWidgetState {
    fn default() -> Self {
        Self {
            within: Time(NaiveTime::from_hms_opt(3, 0, 0).unwrap()),
        }
    }
}

pub fn UpcomingEventsWidget(cx: &ScopeState, _: WidgetSize) -> Element {
    use_interval(cx, std::time::Duration::new(60, 0));
    let state = use_shared_state::<UpcomingEventsWidgetState>(cx)?;
    let within_value = state.read().within;
    let now = Local::now();
    let time_now = now.time();
    let schedule = use_shared_state::<Schedule>(cx)?;
    let schedule = schedule.read();
    let events = schedule.get_events_on_date(now.date_naive());
    let events = events.filter_map(|(i, e)| {
        let within = within_value.as_duration();
        let time_pair = e.time_pair().get();
        let low = time_pair.0 .0;
        let high = time_pair.1 .0;
        (time_now + within >= low && high >= time_now).then_some(i)
    });
    let events = events.map(|i| {
        render! {
            EventTitleButton {
                i: i
            }
        }
    });
    let update = cx.schedule_update();

    render! {
        div {
            class: "card flex-1 rounded-lg bg-base-100 shadow-xl overflow-clip",
            div {
                class: "card-body p-[0.5rem]",
                div {
                    class: "card-title",
                    span {
                        class: "text-sm",
                        "Upcoming in"
                    }
                    form {
                        onsubmit: move |e| {
                            let values = &e.data.values;
                            let within_value = &values["within"][0];
                            let within_value = grammar::TimeParser::new().parse(within_value);

                            if let Ok(within_value) = within_value {
                                state.with_mut(|s| s.within = Time(within_value));
                            } else {
                                update();
                            }
                        },
                        input {
                            class: "input input-bordered input-xs w-16",
                            r#type: "text",
                            name: "within",
                            placeholder: "h:min",
                            maxlength: 5,
                            value: within_value.to_string(),
                        }
                        input {
                            r#type: "submit",
                            hidden: true,
                        }
                    }
                }
                div {
                    class: "join join-vertical rounded-none h-0 flex flex-col flex-wrap grow overflow-hidden",
                    events
                }
            }
        }
    }
}
