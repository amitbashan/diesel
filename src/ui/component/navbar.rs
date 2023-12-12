use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::ui::*;

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
                        svg {
                            class: "inline-block w-5 h-5 stroke-current fill-none",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            path {
                                d: "M4 6h16M4 12h16M4 18h16",
                                stroke_width: 2,
                            }
                        }
                    }
                    ul {
                        class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52",
                        tabindex: 0,
                        li {
                            Link {
                                to: Route::Index {},
                                svg {
                                    class: "inline-block w-5 h-5 stroke-current fill-none",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 256 256",
                                    path {
                                        d: "M152,208V160a8,8,0,0,0-8-8H112a8,8,0,0,0-8,8v48a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V115.5a8.3,8.3,0,0,1,2.6-5.9l80-72.7a8,8,0,0,1,10.8,0l80,72.7a8.3,8.3,0,0,1,2.6,5.9V208a8,8,0,0,1-8,8H160A8,8,0,0,1,152,208Z",
                                        stroke_width: 18,
                                    }
                                }
                                "Home",
                            }
                        }
                        li {
                            Link {
                                to: Route::Calendar {},
                                svg {
                                    class: "inline-block w-5 h-5 stroke-current fill-none",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 20 20",
                                    path {
                                        d: "M16.557,4.467h-1.64v-0.82c0-0.225-0.183-0.41-0.409-0.41c-0.226,0-0.41,0.185-0.41,0.41v0.82H5.901v-0.82c0-0.225-0.185-0.41-0.41-0.41c-0.226,0-0.41,0.185-0.41,0.41v0.82H3.442c-0.904,0-1.64,0.735-1.64,1.639v9.017c0,0.904,0.736,1.64,1.64,1.64h13.114c0.904,0,1.64-0.735,1.64-1.64V6.106C18.196,5.203,17.461,4.467,16.557,4.467 M17.377,15.123c0,0.453-0.366,0.819-0.82,0.819H3.442c-0.453,0-0.82-0.366-0.82-0.819V8.976h14.754V15.123z M17.377,8.156H2.623V6.106c0-0.453,0.367-0.82,0.82-0.82h1.639v1.23c0,0.225,0.184,0.41,0.41,0.41c0.225,0,0.41-0.185,0.41-0.41v-1.23h8.196v1.23c0,0.225,0.185,0.41,0.41,0.41c0.227,0,0.409-0.185,0.409-0.41v-1.23h1.64c0.454,0,0.82,0.367,0.82,0.82V8.156z",
                                    }
                                }
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
                        svg {
                            class: "inline-block w-5 h-5 stroke-current fill-none",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            path {
                                d: "M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z",
                                stroke_width: 2,
                            }
                        }
                    }
                }
            }
        }
    }
}
