use dioxus::prelude::*;

use crate::ui::Theme;

#[component]
pub fn ThemeCard(cx: Scope, theme: &'static str) -> Element {
    let theme_state = use_shared_state::<Theme>(cx)?;
    let chosen = (&theme_state.read().0 == theme)
        .then_some("!outline-base-content")
        .unwrap_or_default();

    render! {
        div {
            class: "border-base-content/20 hover:border-base-content/40 overflow-hidden rounded-lg border outline outline-2 outline-offset-2 outline-transparent {chosen}",
            div {
                class: "text-base-content w-full cursor-pointer font-sans bg-base-100",
                "data-theme": *theme,
                onclick: move |_| {
                    theme_state.with_mut(|s| s.0 = theme);
                },
                div {
                    class: "grid grid-cols-5 grid-rows-3",
                    div { class: "row-span-2 row-start-1 col-start-1 bg-base-200" },
                    div { class: "col-start-1 row-start-3 bg-base-300" },
                    div {
                        class: "flex flex-col gap-1 p-2 row-span-3 col-span-4 row-start-1 col-start-2 bg-base-100",
                        div {
                            class: "font-bold",
                            theme
                        }
                        div {
                            class: "flex flex-wrap gap-1",
                            div {
                                class: "flex aspect-square w-5 items-center justify-center rounded bg-primary",
                                div {
                                    class: "text-primary-content text-sm font-bold",
                                    "A"
                                }
                            }
                            div {
                                class: "flex aspect-square w-5 items-center justify-center rounded bg-secondary",
                                div {
                                    class: "text-secondary-content text-sm font-bold",
                                    "A"
                                }
                            }
                            div {
                                class: "flex aspect-square w-5 items-center justify-center rounded bg-accent",
                                div {
                                    class: "text-accent-content text-sm font-bold",
                                    "A"
                                }
                            }
                            div {
                                class: "flex aspect-square w-5 items-center justify-center rounded bg-neutral",
                                div {
                                    class: "text-neutral-content text-sm font-bold",
                                    "A"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
