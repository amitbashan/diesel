use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::ui::{component::svg, *};

#[component]
pub fn Navbar<'a>(cx: Scope<'a>, center: Option<Element<'a>>, end: Option<Element<'a>>) -> Element {
    let version = env!("CARGO_PKG_VERSION");

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
                    "{version}"
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
                    class: "flex-none",
                    button {
                        class: "btn btn-square btn-ghost",
                        svg::Ellipsis {}
                    }
                }
            }
        }
    }
}
