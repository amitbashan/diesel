use dioxus::prelude::*;

use super::ThemeCard;
use crate::ui::{Theme, THEMES};

fn Configuration(cx: Scope) -> Element {
    render! {
        "Configuration"
    }
}

fn Appearance(cx: Scope) -> Element {
    let theme_state = use_shared_state::<Theme>(cx)?;
    let cards = THEMES.iter().map(|theme| {
        render! {
            ThemeCard {
                theme: theme,
            }
        }
    });

    render! {
        div {
            class: "flex justify-between pb-2",
            span {
                class: "text-lg font-bold",
                "Theme"
            }
            button {
                class: "btn btn-sm",
                onclick: move |_| {
                    theme_state.with_mut(|s| s.auto());
                },
                "Auto"
            }
        }
        div {
            class: "max-h-40 overflow-y-auto",
            div {
                class: "grid grid-cols-2 gap-2 p-2",
                cards
            }
        }
    }
}

pub fn Settings(cx: Scope) -> Element {
    const VIEWS: [fn(Scope) -> Element; 2] = [Configuration, Appearance];
    const VIEWS_LABELS: [&str; 2] = ["Configuration", "Appearance"];
    let page = use_state(cx, || 0);
    let view = VIEWS[*page.get()];
    let view_buttons = VIEWS_LABELS.iter().enumerate().map(|(i, view)| {
        let chosen = (&i == page.get())
            .then_some("btn-active")
            .unwrap_or_default();
        render! {
            button {
                class: "btn join-item {chosen}",
                onclick: move |_| {
                    page.set(i);
                },
                view
            }
        }
    });

    render! {
        div {
            class: "grid grid-cols-4",
            div {
                class: "w-14",
                div {
                    class: "join join-vertical rounded-l-lg rounded-r-none border-r-2 border-neutral",
                    view_buttons
                }
            }
            div {
                class: "col-span-3 pl-1.5",
                view(cx),
            }
        }
    }
}
