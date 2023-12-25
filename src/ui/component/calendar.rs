use std::rc::Rc;

use chrono::prelude::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::Modal;
use crate::{
    ql::*,
    schedule::{event::Event, *},
    ui::{
        view::event::{
            DEFAULT_ERROR_STATE, INPUT_ERROR, PREDICATE_ERROR_STATE, TIMESPAN_ERROR_STATE,
            TITLE_ERROR_STATE,
        },
        Route,
    },
};

#[component]
fn NewEventModal(cx: Scope, open: UseState<bool>, state: UseState<Option<NaiveDate>>) -> Element {
    let schedule = use_shared_state::<Schedule>(cx)?;
    let date = (*state.get())?;
    let error_state = use_ref(cx, || DEFAULT_ERROR_STATE);
    let reset_state = || {
        state.set(None);
        open.set(false);
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

    let predicate_expression = Predicate::Equality(
        Expression::Placeholder(PlaceholderUnit::Date),
        Expression::Date(date),
    );

    render! {
        Modal {
            open: open.clone(),
            span {
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
                    let time_pair = &values["timepair"][0];
                    let time_pair = grammar::TimePairParser::new().parse(time_pair.as_str());

                    if predicate.is_err() {
                        set_error_state(PREDICATE_ERROR_STATE);
                    }

                    if time_pair.is_err() {
                        set_error_state(TIMESPAN_ERROR_STATE);
                    }

                    if let (Ok(predicate), Ok(time_pair)) = (predicate, time_pair) {
                        let title = &values["title"][0];
                        if title.is_empty() {
                            set_error_state(TITLE_ERROR_STATE);
                            return;
                        }
                        let title = title.clone();
                        let description = values["description"][0].clone();
                        let event = Event::new(title, description, predicate, time_pair);
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
                                name: "timepair",
                                r#type: "text",
                                maxlength: 11,
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
    }
}

#[component]
pub fn EventTitleButton(cx: Scope, i: usize) -> Element {
    let navigator = use_navigator(cx);
    let schedule = use_shared_state::<Schedule>(cx)?;
    let schedule = schedule.read();
    let event = schedule.get_event(*i)?;
    let title = event.title();
    let title = title.borrow();
    let title = title.as_str();

    render! {
        button {
            class: "join-item w-full btn btn-outline justify-start h-5 min-h-5 pl-2 pr-2",
            onclick: move |_| {
                navigator.push(Route::EventInstance { i: *i });
            },
            span { class: "truncate", "{title}" }
        }
    }
}

#[component]
fn CalendarCard(
    cx: Scope,
    date: NaiveDate,
    modal_open: UseState<bool>,
    modal_state: UseState<Option<NaiveDate>>,
) -> Element {
    let now = Local::now().date_naive();
    let bordered = (&now == date)
        .then_some("card-bordered border-neutral")
        .unwrap_or_default();
    let schedule = use_shared_state::<Schedule>(cx)?.read();
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
            class: "card bg-base-100 shadow-xl overflow-clip {bordered}",
            ondblclick: move |_| {
                modal_open.set(true);
                modal_state.set(Some(*date));
            },
            div {
                class: "card-body p-[0.5rem]",
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
    let modal_open = use_state(cx, || false);
    let modal_state = use_state(cx, || None::<NaiveDate>);
    let cards =
        days.map(|d| render! { CalendarCard { date: d, modal_open: modal_open.clone(), modal_state: modal_state.clone() } });

    render! {
        div {
            class: "flex items-start rounded bg-base-200 h-full",
            div {
                class: "flex-1 grid grid-cols-7 grid-rows-5 gap-2 p-2 h-full",
                cards
            }
        }
        NewEventModal { open: modal_open.clone(), state: modal_state.clone() }
    }
}
