use chrono::{prelude::*, Months};
use dioxus::prelude::*;

use crate::ui::component::{Layout, MonthlyCalendar, Navbar};

pub fn Calendar(cx: Scope) -> Element {
    let state = use_state(cx, || Local::now());
    let year = state.year();
    let month = state.month();
    let named_month = Month::try_from(month as u8).unwrap();

    render! {
        Layout {
            navbar: render! { Navbar {} },
            div {
                class: "flex justify-between mb-1",
                div {
                    span {
                        class: "font-bold text-lg",
                        "{named_month:?} "
                    }
                    span {
                        class: "text-lg",
                        "{year}"
                    }
                }
                div {
                    class: "join",
                    button {
                        class: "btn btn-sm join-item",
                        onclick: move |_| {
                            state.modify(|s| s.checked_sub_months(Months::new(1)).unwrap())
                        },
                        "<"
                    }
                    button {
                        class: "btn btn-sm join-item",
                        onclick: move |_| {
                            state.set(Local::now());
                        },
                        "Now"
                    }
                    button {
                        class: "btn btn-sm join-item",
                        onclick: move |_| {
                            state.modify(|s| s.checked_add_months(Months::new(1)).unwrap())
                        },
                        ">"
                    }
                }
            }
            MonthlyCalendar { year: year, month: month }
        }
    }
}
