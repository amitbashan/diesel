use std::rc::Rc;

use chrono::{prelude::*, Datelike};
use dioxus::prelude::*;

use crate::schedule::*;

#[component]
fn NewEventModal(cx: Scope, state: UseState<Option<NaiveDate>>) -> Element {
    const DURATION_PLACEHOLDER: &str = "d/mo/y:(h-min)?";
    if let Some(date) = state.get() {
        render! {
            div {
                class: "modal modal-open",
                div {
                    class: "modal-box",
                    h3 {
                        class: "font-bold text-lg",
                        "Schedule a new event",
                    }
                    div {
                        class: "form-control pt-4",
                        div {
                            class: "grid grid-cols-3 gap-2",
                            div {
                                div {
                                    class: "label",
                                    span {
                                        class: "label-text",
                                        "Date"
                                    }
                                }
                                input {
                                    class: "input input-sm input-bordered w-full",
                                    placeholder: "d/mo/y",
                                }
                            }
                            div {
                                class: "col-span-2",
                                div {
                                    class: "label",
                                    span {
                                        class: "label-text",
                                        "Duration"
                                    }
                                }
                                div {
                                    class: "join",
                                    div {
                                        class: "tooltip tooltip-bottom",
                                        "data-tip": "Start",
                                        input {
                                            class: "join-item input input-sm input-bordered w-full",
                                            placeholder: DURATION_PLACEHOLDER,
                                        }
                                    }
                                    div {
                                        class: "tooltip tooltip-bottom",
                                        "data-tip": "End",
                                        input {
                                            class: "join-item input input-sm input-bordered w-full",
                                            placeholder: DURATION_PLACEHOLDER,
                                        }
                                    }
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
                                    class: "input input-sm input-bordered w-full",
                                    placeholder: "Add title…",
                                }
                            }
                            div {
                                class: "col-span-3",
                                div {
                                    class: "label",
                                    span {
                                        class: "label-text",
                                        "Description"
                                    }
                                }
                                textarea {
                                    class: "textarea textarea-bordered w-full",
                                    placeholder: "Add description…",
                                }
                            }
                        }
                    }
                    div {
                        class: "modal-action",
                        button {
                            class: "btn",
                            "Schedule"
                        }
                    }
                }
                div {
                    class: "modal-backdrop cursor-pointer",
                    onclick: move |_| {
                        state.set(None);
                    }
                }
            }
        }
    } else {
        None
    }
}

#[component]
fn EventTitleButton(cx: Scope, title: Rc<String>) -> Element {
    render! {
        button {
            class: "join-item w-full btn btn-xxs btn-outline justify-start",
            span { class: "truncate", "{title}" }
        }
    }
}

#[component]
fn CalendarCard(cx: Scope, date: NaiveDate, modal_state: UseState<Option<NaiveDate>>) -> Element {
    let now = Local::now().date_naive();
    let bordered = (now.day0() == date.day0())
        .then_some("card-bordered border-neutral")
        .unwrap_or_default();
    let schedule = use_shared_state::<Schedule>(cx).unwrap().read();
    let d = date.day0() + 1;
    let weekday = date.weekday();
    let events = schedule.events_on_date(&date);
    let displayed_events: Vec<_> = events
        .map(|e| render! { EventTitleButton { title: e.title() }})
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
            ondblclick: move |e| {
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
    let days = date.iter_days().take_while(|d| d.month0() + 1 == *month);
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
