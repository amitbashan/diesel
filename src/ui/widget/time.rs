use chrono::Timelike;
use dioxus::prelude::*;

use crate::hook::use_interval;

use super::{WidgetSize, WidgetStates};

pub fn TimeWidget(
    cx: &ScopeState,
    widget_size: WidgetSize,
    _: UseSharedState<WidgetStates>,
) -> Element {
    use_interval(cx, std::time::Duration::new(1, 0));
    let time = chrono::Local::now();
    let h = time.hour();
    let m = time.minute();
    let s = time.second();
    let n = time.nanosecond();

    render! {
        div {
            class: "flex flex-1 h-full rounded-lg justify-center items-center text-xl shadow-xl bg-base-100 select-none cursor-default",
            match widget_size {
                WidgetSize::Small => render! { "{h}:{m}" },
                WidgetSize::Medium => render! { "{h}:{m}:{s}" },
                WidgetSize::Large => render! { "{h}:{m}:{s}:{n}" },
            }
        }
    }
}
