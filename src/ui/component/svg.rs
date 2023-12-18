use dioxus::prelude::*;

pub fn Trash(cx: Scope) -> Element {
    render! {
        svg {
            class: "w-5 h-5 stroke-current fill-current",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 16 16",
            path {
                d: "M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5zm3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0z",
            }
        }
    }
}

pub fn Edit(cx: Scope) -> Element {
    render! {
        svg {
            class: "w-5 h-5 stroke-current fill-current",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            path {
                d: "M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83",
            }
        }
    }
}

pub fn StackedWindows(cx: Scope) -> Element {
    render! {
        svg {
            class: "inline-block w-5 h-5 stroke-current fill-none",
            view_box: "0 0 20 20",
            path {
                d: "M17.391,2.406H7.266c-0.232,0-0.422,0.19-0.422,0.422v3.797H3.047c-0.232,0-0.422,0.19-0.422,0.422v10.125c0,0.232,0.19,0.422,0.422,0.422h10.125c0.231,0,0.422-0.189,0.422-0.422v-3.797h3.797c0.232,0,0.422-0.19,0.422-0.422V2.828C17.812,2.596,17.623,2.406,17.391,2.406 M12.749,16.75h-9.28V7.469h3.375v5.484c0,0.231,0.19,0.422,0.422,0.422h5.483V16.75zM16.969,12.531H7.688V3.25h9.281V12.531z",
            }
        }
    }
}

pub fn Burger(cx: Scope) -> Element {
    render! {
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
}

pub fn Home(cx: Scope) -> Element {
    render! {
        svg {
            class: "inline-block w-5 h-5 stroke-current fill-none",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 256 256",
            path {
                d: "M152,208V160a8,8,0,0,0-8-8H112a8,8,0,0,0-8,8v48a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V115.5a8.3,8.3,0,0,1,2.6-5.9l80-72.7a8,8,0,0,1,10.8,0l80,72.7a8.3,8.3,0,0,1,2.6,5.9V208a8,8,0,0,1-8,8H160A8,8,0,0,1,152,208Z",
                stroke_width: 18,
            }
        }
    }
}

pub fn Calendar(cx: Scope) -> Element {
    render! {
        svg {
            class: "inline-block w-5 h-5 stroke-current fill-none",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 20 20",
            path {
                d: "M16.557,4.467h-1.64v-0.82c0-0.225-0.183-0.41-0.409-0.41c-0.226,0-0.41,0.185-0.41,0.41v0.82H5.901v-0.82c0-0.225-0.185-0.41-0.41-0.41c-0.226,0-0.41,0.185-0.41,0.41v0.82H3.442c-0.904,0-1.64,0.735-1.64,1.639v9.017c0,0.904,0.736,1.64,1.64,1.64h13.114c0.904,0,1.64-0.735,1.64-1.64V6.106C18.196,5.203,17.461,4.467,16.557,4.467 M17.377,15.123c0,0.453-0.366,0.819-0.82,0.819H3.442c-0.453,0-0.82-0.366-0.82-0.819V8.976h14.754V15.123z M17.377,8.156H2.623V6.106c0-0.453,0.367-0.82,0.82-0.82h1.639v1.23c0,0.225,0.184,0.41,0.41,0.41c0.225,0,0.41-0.185,0.41-0.41v-1.23h8.196v1.23c0,0.225,0.185,0.41,0.41,0.41c0.227,0,0.409-0.185,0.409-0.41v-1.23h1.64c0.454,0,0.82,0.367,0.82,0.82V8.156z",
            }
        }
    }
}

pub fn Ellipsis(cx: Scope) -> Element {
    render! {
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

pub fn Check(cx: Scope) -> Element {
    render! {
        svg {
            class: "w-5 h-5 fill-current",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 16 16",
            path {
                d: "M13.854 3.646a.5.5 0 0 1 0 .708l-7 7a.5.5 0 0 1-.708 0l-3.5-3.5a.5.5 0 1 1 .708-.708L6.5 10.293l6.646-6.647a.5.5 0 0 1 .708 0z"
            }
        }
    }
}
