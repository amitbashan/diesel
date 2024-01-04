use std::borrow::Cow;

use dioxus::prelude::*;

#[component]
pub fn Modal<'a>(
    cx: Scope<'a>,
    class: Option<&'a str>,
    open: UseState<bool>,
    children: Element<'a>,
) -> Element {
    let class = class.unwrap_or_default();
    let open_class = open.get().then_some("modal-open").unwrap_or_default();

    render! {
        div {
            class: "modal {open_class}",
            div {
                class: "modal-box {class}",
                children
            }
            div {
                class: "modal-backdrop cursor-pointer",
                onclick: move |_| {
                    open.set(false);
                }
            }
        }
    }
}

#[component]
pub fn ErrorModal<'a>(
    cx: Scope<'a>,
    open: Option<UseState<bool>>,
    action: Option<Element<'a>>,
    description: Option<Cow<'a, str>>,
    error: Option<Cow<'a, str>>,
) -> Element {
    let open = open
        .as_ref()
        .map(|s| *s.get())
        .unwrap_or(true)
        .then_some("modal-open")
        .unwrap_or_default();

    render! {
        dialog {
            class: "modal {open}",
            div {
                class: "modal-box",
                article {
                    class: "prose",
                    h3 {
                        "Error"
                    }
                    p { description.as_ref().unwrap_or(&Cow::Borrowed("No description provided.")) }
                    if let Some(e) = error {
                        render! {
                            div {
                                class: "border border-neutral font-mono bg-base-300",
                                e
                            }
                        }
                    }
                }
                div {
                    class: "modal-action",
                    if let Some(action) = action {
                        action
                    }
                }
            }
            div {
                class: "modal-backdrop",
            }
        }
    }
}
