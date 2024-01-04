use std::{
    borrow::{Borrow, Cow},
    fs,
};

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    configuration::{Configuration, ConfigurationManager},
    hook::use_save,
    ui::{
        component::{ErrorModal, ThemeDropdown},
        Route,
    },
};

pub fn Setup(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let cfg_manager = use_shared_state::<ConfigurationManager>(cx)?;
    let path = &cfg_manager.read().path;
    let update = cx.schedule_update();
    let reset = move || {
        cfg_manager.with_mut(|s| s.path = None);
        update();
    };

    if let Some(path) = path {
        let cfg = Configuration::import(path);

        match cfg {
            Ok(Some(cfg)) => {
                cfg.load(cx);
                navigator.push(Route::Index {});
                None
            }
            Ok(_) => render! {
                ErrorModal {
                    action: render! {
                        button {
                            class: "btn",
                            onclick: move |_| {
                                reset();
                            },
                            "Setup"
                        }
                    },
                    description: Cow::Borrowed("Failed to read user configuration."),
                }
            },
            Err(e) => render! {
                ErrorModal {
                    action: render! {
                        button {
                            class: "btn",
                            onclick: move |_| {
                                reset();
                            },
                            "Setup"
                        }
                    },
                    description: Cow::Borrowed("Encountered an I/O error while trying to read user configuration:"),
                    error: Cow::Owned(e.to_string()),
                }
            },
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
                                        cfg_manager.with_mut(|s| s.path = Some(file));
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
                                    "Create a new configuration"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn New(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let save = use_save(cx);
    let cfg_manager = use_shared_state::<ConfigurationManager>(cx)?;
    let modal_state = use_state(cx, || false);
    let description = use_state(cx, || Cow::Borrowed(""));
    let error = use_state(cx, || Cow::Borrowed(""));
    let page = use_state(cx, || 0);
    let path_state = use_state(cx, || None::<String>);
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
                    class: "input input-bordered w-full font-mono sm:max-w-md lg:max-w-lg",
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
        2 => {
            let path = path_state.get().as_ref().unwrap();
            if let Err(e) = fs::File::create(path) {
                page.modify(|p| p - 1);
                description.set(Cow::from(
                    "Encountered an I/O error while creating your configuration:",
                ));
                error.set(Cow::Owned(e.to_string()));
                modal_state.set(true);
                None
            } else {
                cfg_manager.with_mut(|s| s.path = Some(path.clone()));
                save();
                navigator.push(Route::Setup {});
                None
            }
        }
        _ => None,
    };

    let description: Cow<_> = Borrow::<str>::borrow(description.get().as_ref()).into();
    let error: Cow<_> = Borrow::<str>::borrow(error.get().as_ref()).into();

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
                render! {
                    div {
                        class: "flex justify-center items-center",
                        button {
                            class: "btn btn-outline btn-sm join-item",
                            onclick: move |_| {
                                if *page.get() > 0 {
                                    page.modify(|p| p - 1);
                                } else {
                                    navigator.push(Route::Setup {});
                                }
                            },
                            "←"
                        }
                    }
                }
            }
        }
        ErrorModal {
            action: render! {
                button {
                    class: "btn",
                    onclick: move |_| {
                        modal_state.set(false);
                    },
                    "←"
                }
            },
            open: modal_state.clone(),
            description: description,
            error: error
        }
    }
}
