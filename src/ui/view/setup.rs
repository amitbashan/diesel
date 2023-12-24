use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::ui::{
    self,
    component::{ErrorModal, ThemeDropdown},
    Theme,
};

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    Index {},
    #[route("/new")]
    New {},
}

pub fn Setup(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn Index(cx: Scope) -> Element {
    let cfg_input = use_state(cx, || None::<String>);

    if let Some(path) = cfg_input.get() {
        render! {
            "wip",
        }
    } else {
        render! {
            div {
                class: "flex flex-1 justify-center items-center h-screen",
                div {
                    class: "grid grid-cols-1 grid-rows-2 w-full sm:gap-6 md:gap-10 lg:gap-14",
                    p {
                        class: "font-bold text-center sm:text-2xl md:text-4xl lg:text-6xl",
                        "Setup"
                    }
                    div {
                        class: "flex w-full justify-center",
                        input {
                            class: "hidden",
                            id: "cfg-file-input",
                            r#type: "file",
                            onchange: move |e| {
                                if let Some(file_engine) = &e.files {
                                    let files = file_engine.files();
                                    if files.len() > 0 {
                                        let file = file_engine.files().remove(0);
                                        cfg_input.set(Some(file));
                                    }
                                }
                            },
                        }
                        div {
                            class: "join",
                            label {
                                class: "btn btn-outline join-item",
                                r#for: "cfg-file-input",
                                "Import",
                            }
                            Link {
                                to: "/new",
                                button {
                                    class: "btn btn-outline join-item",
                                    "Create new configuration"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn New(cx: Scope) -> Element {
    let page = use_state(cx, || 0);
    let path_state = use_state(cx, || None::<String>);
    let theme_state = use_shared_state::<Theme>(cx)?;
    let view = match page.get() {
        0 => render! {
            form {
                class: "flex flex-col justify-center items-center gap-4",
                onsubmit: move |e| {
                    let values = &e.values;
                    let path = values["path"][0].clone();
                    path_state.set(Some(path));
                    page.modify(|p| p + 1);
                },
                input {
                    class: "input input-bordered w-full sm:max-w-md lg:max-w-lg",
                    name: "path",
                    r#type: "text",
                    placeholder: "Enter path for user configuration…"
                }
                button {
                    class: "btn sm:max-w-xs lg:max-w-lg",
                    r#type: "submit",
                    "Continue"
                }
            }
        },
        1 => render! {
            div {
                class: "flex flex-col justify-center items-center gap-8",
                p {
                    class: "font-bold sm:text-xl md:text-2xl lg:text-4xl",
                    "Choose a theme",
                }
                ThemeDropdown {}
                button {
                    class: "btn sm:max-w-xs md:max-w-md lg:max-w-lg",
                    onclick: move |_| {
                        page.modify(|p| p + 1);
                    },
                    "Continue"
                }
            }
        },
        2 => None,
        _ => None,
    };

    if *page.get() == 3 {
        ui::UI(cx)
    } else {
        render! {
            div {
                class: "flex flex-1 justify-center items-center h-screen",
                div {
                    class: "flex flex-col w-full gap-4",
                    p {
                        class: "font-bold text-center sm:text-2xl md:text-4xl lg:text-6xl mb-4",
                        "Setup"
                    }
                    view,
                    if *page.get() > 0 {
                        render! {
                            div {
                                class: "flex justify-center items-center",
                                button {
                                    class: "btn btn-outline btn-sm join-item",
                                    onclick: move |_| {
                                        page.modify(|p| p - 1);
                                    },
                                    "←"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
