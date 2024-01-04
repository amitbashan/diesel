use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::ThemeCard;
use crate::{
    configuration::{Configuration, ConfigurationManager},
    ui::{Route, Theme, THEMES},
};

fn Configuration<'a>(
    cx: &'a ScopeState,
    cfg_manager: &'a UseSharedState<ConfigurationManager>,
) -> Element<'a> {
    let path = cfg_manager.with(|cfg| {
        cfg.path
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string()
    });
    let navigator = use_navigator(cx);

    render! {
        div {
            class: "flex flex-col gap-1 m-2",
            div {
                class: "flex justify-between w-full",
                p {
                    class: "text-lg font-bold",
                    "Configuration"
                }
                button {
                    class: "btn btn-sm",
                    onclick: move |_| {
                        Configuration::try_save(&cfg_manager).expect("failed to save configuration");
                        cfg_manager.with_mut(|cfg| cfg.path = None);
                        navigator.push(Route::Setup {  });
                    },
                    "Logout"
                }
            }
            div {
                p {
                    class: "text-md",
                    "Path: "
                    input {
                        class: "input input-bordered input-sm w-full max-w-xs font-mono",
                        readonly: true,
                        value: path,
                    }
                }
            }
        }
    }
}

fn Appearance<'a>(cx: &'a ScopeState, _: &'a UseSharedState<ConfigurationManager>) -> Element<'a> {
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
            class: "flex justify-between m-2",
            p {
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
            class: "max-h-64 overflow-y-auto",
            div {
                class: "grid grid-cols-2 gap-2 p-2",
                cards
            }
        }
    }
}

pub fn Settings(cx: Scope) -> Element {
    const VIEWS: [for<'a> fn(
        &'a ScopeState,
        &'a UseSharedState<ConfigurationManager>,
    ) -> Element<'a>; 2] = [Configuration, Appearance];
    const VIEWS_LABELS: [&str; 2] = ["Configuration", "Appearance"];
    let cfg_manager = use_shared_state::<ConfigurationManager>(cx)?;
    let page = use_state(cx, || 0);
    let views = VIEWS.iter().enumerate().map(|(i, view)| {
        render! {
            div {
                hidden: page.get() != &i,
                view(cx, cfg_manager),
            }
        }
    });
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
                class: "col-span-3 pl-1.5 overflow-hidden",
                views
            }
        }
    }
}
