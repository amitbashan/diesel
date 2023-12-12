use dioxus::prelude::*;

#[component]
pub fn Layout<'a>(cx: Scope<'a>, children: Element<'a>, navbar: Element<'a>) -> Element {
    render! {
        div {
            class: "flex flex-col h-screen p-4",
            div {
                class: "mb-4",
                navbar,
            },
            children,
        }
    }
}
