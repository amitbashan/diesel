use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::component::svg;

use super::{WidgetSize, WidgetStates};

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct WeatherWidgetState {
    #[serde(skip)]
    settings: bool,
}

pub fn WeatherWidget(
    cx: &ScopeState,
    widget_size: WidgetSize,
    widget_states: UseSharedState<WidgetStates>,
) -> Element {
    let state = widget_states.with(|s| s.weather);
    let settings = state.settings;

    if settings {
        render! {
            div {
                class: "flex flex-1 h-full rounded-lg text-xl shadow-xl bg-base-100",
                button {
                    class: "tn btn-xs btn-square",
                    onclick: move |_| {
                        widget_states.with_mut(|s| s.weather.settings = !s.weather.settings);
                    },
                    svg::Gear {}
                }
                "Settings"
            }
        }
    } else {
        render! {
            div {
                class: "flex flex-1 h-full rounded-lg text-xl shadow-xl bg-base-100 ",
                button {
                    class: "btn btn-xs btn-square",
                    onclick: move |_| {
                        widget_states.with_mut(|s| s.weather.settings = !s.weather.settings);
                    },
                    svg::Gear {}
                }
                "Weather"
            }
        }
    }
}
