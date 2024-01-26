use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    ui::{
        component::{svg, Modal, Settings},
        *,
    },
    VERSION,
};

#[component]
pub fn Navbar<'a>(cx: Scope<'a>, center: Option<Element<'a>>, end: Option<Element<'a>>) -> Element {
    let settings_modal_state = use_state(cx, || false);

    render! {
        div {
            class: "navbar rounded-box shadow-xl bg-base-200",
            div {
                class: "navbar-start",
                div {
                    class: "dropdown",
                    button {
                        class: "btn btn-square btn-ghost",
                        tabindex: 0,
                        svg::Burger {}
                    }
                    ul {
                        class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52",
                        tabindex: 0,
                        li {
                            Link {
                                to: Route::Index {},
                                svg::Home {},
                                "Home",
                            }
                        }
                        li {
                            Link {
                                to: Route::Calendar {},
                                svg::Calendar {},
                                "Calendar",
                            }
                        }
                    }
                }
                button {
                    class: "btn btn-ghost text-xl",
                    "Diesel"
                }
                span {
                    class: "font-mono text-xs",
                    VERSION,
                }
            }
            div {
                class: "navbar-center",
                if let Some(center) = center {
                    center
                }
            }
            div {
                class: "navbar-end",
                if let Some(end) = end {
                    end
                }
                div {
                    class: "dropdown dropdown-bottom dropdown-end",
                    button {
                        class: "btn btn-square btn-ghost",
                        tabindex: 0,
                        svg::Ellipsis {}
                    }
                    ul {
                        class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-40",
                        tabindex: 0,
                        li {
                            a {
                                onclick: move |_| {
                                    settings_modal_state.modify(|s| !s);
                                },
                                svg::Gear {},
                                "Settings",
                            }
                        }
                    }
                }
            }
        }
        Modal {
            class: "h-80 p-2",
            open: settings_modal_state.clone(),
            Settings {}
        }
    }
}
