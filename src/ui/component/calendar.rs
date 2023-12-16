use std::rc::Rc;

use chrono::{prelude::*, Datelike, Duration};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    ql::*,
    schedule::{Event, *},
    ui::Route,
};

#[component]
fn NewEventModal(cx: Scope, state: UseState<Option<NaiveDate>>) -> Element {
    const TITLE_ERROR_STATE: usize = 0;
    const PREDICATE_ERROR_STATE: usize = 1;
    const TIMESPAN_ERROR_STATE: usize = 2;
    const INPUT_ERROR: &str = "input-error";
    let schedule = use_shared_state::<Schedule>(cx).unwrap();
    let error_state = use_ref(cx, || [false; 3]);
    let reset_state = || {
        state.set(None);
        error_state.with_mut(|s| s.iter_mut().for_each(|f| *f = false));
    };
    let get_error_state = move |i| {
        let flag: bool = error_state.read()[i];
        flag.then_some(INPUT_ERROR).unwrap_or_default()
    };
    let set_error_state = |i| {
        error_state.with_mut(|s| s[i] = true);
    };
    let title_input_error_state = get_error_state(TITLE_ERROR_STATE);
    let predicate_input_error_state = get_error_state(PREDICATE_ERROR_STATE);
    let timespan_input_error_state = get_error_state(TIMESPAN_ERROR_STATE);

    if let Some(date) = state.get() {
        let predicate_expression = Predicate::Equality(
            Expression::Placeholder(PlaceholderUnit::Date),
            Expression::Date(*date),
        );
        render! {
            div {
                class: "modal modal-open",
                div {
                    class: "modal-box",
                    h3 {
                        class: "font-bold text-lg",
                        "Schedule a new event",
                    }
                    form {
                        onsubmit: move |e| {
                            let values = &e.data.values;
                            let predicate = &values["predicate"][0];
                            let predicate = if predicate.is_empty() {
                                Ok(predicate_expression)
                            } else {
                                grammar::PredicateParser::new().parse(predicate.as_str())
                            };
                            let time_span = &values["timespan"][0];
                            let time_span = grammar::TimeSpanParser::new().parse(time_span.as_str());

                            if predicate.is_err() {
                                set_error_state(PREDICATE_ERROR_STATE);
                            }

                            if time_span.is_err() {
                                set_error_state(TIMESPAN_ERROR_STATE);
                            }

                            if let (Ok(predicate), Ok(time_span)) = (predicate, time_span) {
                                let title = &values["title"][0];
                                if title.is_empty() {
                                    set_error_state(TITLE_ERROR_STATE);
                                    return;
                                }
                                let title = title.clone();
                                let description = values["description"][0].clone();
                                let ((h1, min1), (h2, min2)) = time_span;
                                let d1 = Duration::hours(h1 as i64) + Duration::minutes(min1 as i64);
                                let d2 = Duration::hours(h2 as i64) + Duration::minutes(min2 as i64);
                                let duration = d2 - d1;
                                let time = (d1 > d2).then_some(time_span.1).unwrap_or(time_span.0);
                                let event = SkeletonEvent::new(title, description, predicate, time, duration);
                                schedule.with_mut(|s| s.schedule_event(Rc::new(event)));
                                reset_state();
                            }
                        },
                        div {
                            class: "form-control pt-4",
                            div {
                                class: "grid grid-cols-2 gap-2",
                                div {
                                    class: "col-span-2",
                                    div {
                                        class: "label",
                                        span {
                                            class: "label-text",
                                            "Predicate"
                                        }
                                    }
                                    input {
                                        class: "input input-sm input-bordered font-mono w-full {predicate_input_error_state}",
                                        name: "predicate",
                                        r#type: "text",
                                        placeholder: predicate_expression.to_string(),
                                    }
                                }
                                div {
                                    div {
                                        class: "label",
                                        span {
                                            class: "label-text",
                                            "Title"
                                        }
                                    }
                                    input {
                                        class: "input input-sm input-bordered w-full {title_input_error_state}",
                                        name: "title",
                                        r#type: "text",
                                        placeholder: "Add title…",
                                    }
                                }
                                div {
                                    div {
                                        class: "label",
                                        span {
                                            class: "label-text",
                                            "Time span"
                                        }
                                    }
                                    input {
                                        class: "input input-sm input-bordered w-full {timespan_input_error_state}",
                                        name: "timespan",
                                        r#type: "text",
                                        placeholder: "h:min-h:min",
                                    }
                                }
                                div {
                                    class: "col-span-2",
                                    div {
                                        class: "label",
                                        span {
                                            class: "label-text",
                                            "Description"
                                        }
                                    }
                                    textarea {
                                        class: "textarea textarea-bordered w-full",
                                        name: "description",
                                        placeholder: "Add description…",
                                    }
                                }
                            }
                        }
                        div {
                            class: "modal-action",
                            button {
                                class: "btn",
                                r#type: "submit",
                                "Schedule"
                            }
                        }
                    }
                }
                div {
                    class: "modal-backdrop cursor-pointer",
                    onclick: move |_| {
                        reset_state();
                    }
                }
            }
        }
    } else {
        None
    }
}

#[component]
fn EventTitleButton(cx: Scope, i: usize) -> Element {
    let navigator = use_navigator(cx);
    let schedule = use_shared_state::<Schedule>(cx).unwrap();
    let schedule = schedule.read();
    let event = schedule.get_event(*i);
    let title = event.title();

    render! {
        button {
            class: "join-item w-full btn btn-xxs btn-outline justify-start",
            onclick: move |_| {
                navigator.push(Route::EventInstance { i: *i });
            },
            span { class: "truncate", "{title}" }
        }
    }
}

#[component]
fn CalendarCard(cx: Scope, date: NaiveDate, modal_state: UseState<Option<NaiveDate>>) -> Element {
    let now = Local::now().date_naive();
    let bordered = (&now == date)
        .then_some("card-bordered border-neutral")
        .unwrap_or_default();
    let schedule = use_shared_state::<Schedule>(cx).unwrap().read();
    let d = date.day();
    let weekday = date.weekday();
    let events = schedule.get_events_on_date(*date);
    let displayed_events: Vec<_> = events
        .map(|(i, _)| render! { EventTitleButton { i: i }})
        .collect();
    let count = displayed_events.len();
    let displayed_events = render! {
        div {
            class: "join join-vertical rounded-none h-0 flex flex-col flex-wrap grow overflow-hidden",
            displayed_events.into_iter()
        }
    };

    render! {
        div {
            class: "card card-compact bg-base-100 shadow-xl overflow-clip {bordered}",
            ondblclick: move |_| {
                modal_state.set(Some(*date));
            },
            div {
                class: "card-body",
                div {
                    class: "card-title justify-between",
                    span {
                        class: "text-sm",
                        "{weekday:?} {d}",
                    }
                    div {
                        class: "badge badge-neutral",
                        "{count}",
                    }
                }
                displayed_events
            }
        }
    }
}

#[component]
pub fn MonthlyCalendar(cx: Scope, year: i32, month: u32) -> Element {
    let date = chrono::NaiveDate::from_ymd_opt(*year, *month, 1).unwrap();
    let days = date.iter_days().take_while(|d| d.month() == *month);
    let modal_state = use_state(cx, || None::<NaiveDate>);
    let cards =
        days.map(|d| render! { CalendarCard { date: d, modal_state: modal_state.clone() } });

    render! {
        div {
            class: "flex items-start rounded bg-base-200 h-full",
            div {
                class: "flex-1 grid grid-cols-7 grid-rows-5 gap-2 p-2 h-full",
                cards
            }
        }
        NewEventModal { state: modal_state.clone() }
    }
}
