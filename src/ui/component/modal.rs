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
