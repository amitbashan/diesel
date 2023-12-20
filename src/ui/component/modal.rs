use dioxus::prelude::*;

#[component]
pub fn Modal<'a>(cx: Scope<'a>, open: UseState<bool>, children: Element<'a>) -> Element {
    let open_class = open.get().then_some("modal-open").unwrap_or_default();
    render! {
        div {
            class: "modal {open_class}",
            div {
                class: "modal-box",
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
