use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    lang::grammar,
    schedule::*,
    ui::{
        component::{svg, Layout, Modal, Navbar},
        Route,
    },
};

pub const TITLE_ERROR_STATE: usize = 0;
pub const PREDICATE_ERROR_STATE: usize = 1;
pub const TIMESPAN_ERROR_STATE: usize = 2;
pub const INPUT_ERROR: &str = "input-error";
pub const DEFAULT_ERROR_STATE: [bool; 3] = [false; 3];

pub fn Event(cx: Scope) -> Element {
    render! {
        Outlet::<Route> {}
    }
}

#[component]
pub fn EventInstance(cx: Scope, i: usize) -> Element {
    let modal_state = use_state(cx, || false);

    render! {
        Layout {
            navbar: render! {
                Navbar {
                    end: render! {
                        button {
                            class: "btn btn-neutral",
                            onclick: move |_| {
                                modal_state.set(true);
                            },
                            svg::Edit {}
                        }
                    }
                }
            },
            EventDisplay { modal_state: modal_state.clone(), i: *i },
        }
    }
}

#[component]
fn EventEditModal(cx: Scope, state: UseState<bool>, i: usize) -> Element {
    const TITLE_ERROR_STATE: usize = 0;
    const PREDICATE_ERROR_STATE: usize = 1;
    const TIMESPAN_ERROR_STATE: usize = 2;
    let schedule = use_shared_state::<Schedule>(cx)?;
    let schedule_ref = schedule.read();
    let event = schedule_ref.get_event(*i)?;
    let title = event.title().borrow().clone();
    let description = event.description().borrow().clone();
    let predicate = event.predicate();
    let predicate = predicate.borrow();
    let predicate = predicate.to_string();
    let time_pair = event.time_pair().get().to_string();
    let navigator = use_navigator(cx);
    let error_state = use_ref(cx, || [false; 3]);
    let reset_state = || {
        state.set(false);
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

    render! {
        Modal {
            open: state.clone(),
            div {
                class: "flex justify-between",
                span {
                    class: "font-bold text-lg",
                    "Edit event"
                }
                button {
                    class: "btn btn-neutral",
                    onclick: move |_| {
                        schedule.with_mut(|s| s.cancel_event(*i));
                        navigator.go_back();
                    },
                    svg::Trash {}
                }
            }
            form {
                onsubmit: move |e| {
                    let values = &e.data.values;
                    let predicate = &values["predicate"][0];
                    let predicate = grammar::ExpressionParser::new().parse(predicate.as_str());
                    let predicate = predicate.ok().map(|p| p.as_predicate()).flatten();
                    let time_pair = &values["timepair"][0];
                    let time_pair = grammar::TimePairParser::new().parse(time_pair.as_str());

                    if predicate.is_none() {
                        set_error_state(PREDICATE_ERROR_STATE);
                    }

                    if time_pair.is_err() {
                        set_error_state(TIMESPAN_ERROR_STATE);
                    }

                    if let (Some(predicate), Ok(time_pair)) = (predicate, time_pair) {
                        let title = &values["title"][0];
                        if title.is_empty() {
                            set_error_state(TITLE_ERROR_STATE);
                            return;
                        }
                        let title = title.clone();
                        let description = values["description"][0].clone();
                        schedule.with_mut(|s| s.edit_event(*i, Some(title), Some(description), Some(predicate), Some(time_pair)));
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
                                value: predicate,
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
                                value: title,
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
                                placeholder: "h:min-h:min",
                                maxlength: 11,
                                value: time_pair
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
                                value: description
                            }
                        }
                    }
                }
                div {
                    class: "modal-action",
                    button {
                        class: "btn",
                        r#type: "submit",
                        svg::Check {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn EventDisplay(cx: Scope, modal_state: UseState<bool>, i: usize) -> Element {
    let schedule = use_shared_state::<Schedule>(cx)?;
    let schedule = schedule.read();
    let event = schedule.get_event(*i)?;
    let title = event.title();
    let title = title.borrow();
    let title = title.as_str();
    let description = event.description();
    let description = description.borrow();
    let description = description.as_str();
    let description = if description.is_empty() {
        render! {
            p {
                class: "italic placeholder-base-content",
                "No description provided."
            }
        }
    } else {
        render! {
            p {
                class: "whitespace-pre-wrap",
                description
            }
        }
    };

    render! {
        div {
            class: "flex-1 rounded p-6",
            article {
                class: "prose w-full",
                max_width: "inherit",
                h3 {
                    class: "font-bold",
                    "{title}",
                }
                description,
            }
        }
        EventEditModal { state: modal_state.clone(), i: *i }
    }
}
