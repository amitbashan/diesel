use dioxus::prelude::*;

use super::{Widget, WidgetDataTransfer, WidgetDragState, WidgetManagerState, WIDGETS};

#[component]
fn PreviewContainer<'a>(
    cx: Scope<'a>,
    data_transfer: UseState<Option<WidgetDataTransfer>>,
    drag: UseState<bool>,
    widget_data: WidgetDataTransfer,
    children: Element<'a>,
) -> Element {
    let size = widget_data.size.size_class();
    render! {
        div {
            class: "carousel-item {size}",
            draggable: true,
            ondragstart: move |_| {
                data_transfer.set(Some(*widget_data));
                drag.set(true);
            },
            ondragend: move |_| {
                data_transfer.set(None);
                drag.set(false);
            },
            children,
        }
    }
}

#[component]
pub fn Drawer<'a>(
    cx: Scope<'a>,
    toggle: UseState<bool>,
    data_transfer: UseState<Option<WidgetDataTransfer>>,
    drag: UseState<bool>,
    children: Element<'a>,
) -> Element {
    let make_preview_container = |widget: Widget, sizes: [bool; 3]| {
        let w = WIDGETS[widget as usize];
        let containers = sizes
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i.into()))
            .map(|size| {
                render! {
                    PreviewContainer {
                        data_transfer: data_transfer.clone(),
                        drag: drag.clone(),
                        widget_data: WidgetDataTransfer { widget, size, source_index: None },
                        w(cx, size)
                    }
                }
            });

        render! {
            div {
                p {
                    class: "text-lg",
                    "{widget:?}"
                }
                div {
                    class: "carousel carousel-center max-w-xs items-center p-4 space-x-4 bg-neutral rounded-box",
                    containers
                },
            }
        }
    };

    render! {
        div {
            class: "drawer drawer-end {toggle}",
            input {
                checked: *toggle.get(),
                class: "drawer-toggle",
                r#type: "checkbox",
            }
            div {
                class: "drawer-content",
                children
            }
            div {
                class: "drawer-side",
                div {
                    class: "drawer-overlay",
                    ondragenter: move |_| {
                        toggle.set(false);
                    },
                    onclick: move |_| {
                        toggle.set(false);
                    }
                }
                div {
                    class: "w-92 min-h-full p-4 bg-base-200",
                    make_preview_container(Widget::Time, [true, true, true])
                }
            }
        }
    }
}

#[component]
pub fn DrawerButton<'a>(
    cx: Scope<'a>,
    toggle: UseState<bool>,
    drag_state: WidgetDragState<'a>,
) -> Element {
    const CLASS: &str = "btn btn-neutral mr-1";
    let wms = use_shared_state::<WidgetManagerState>(cx).unwrap();

    if *drag_state.drag.get() && !toggle.get() {
        render! {
            button {
                class: CLASS,
                prevent_default: "ondragover",
                ondragover: move |e| {
                    e.stop_propagation();
                },
                ondrop: move |_| {
                    if let Some(cell_index) = drag_state.cell_index.get() {
                        let mut wms = wms.write();
                        wms.remove_widget(cell_index);
                    }

                    drag_state.cell_index.set(None);
                    drag_state.drag.set(false);
                },
                svg {
                    class: "w-5 h-5",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "currentColor",
                    view_box: "0 0 16 16",
                    path {
                        d: "M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5zm3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0z",
                    }
                }
            }
        }
    } else {
        render! {
            button {
                class: CLASS,
                onclick: move |_| {
                    toggle.modify(|t| !t);
                },
                svg {
                    class: "inline-block w-5 h-5 stroke-current fill-none",
                    view_box: "0 0 20 20",
                    path {
                        d: "M17.391,2.406H7.266c-0.232,0-0.422,0.19-0.422,0.422v3.797H3.047c-0.232,0-0.422,0.19-0.422,0.422v10.125c0,0.232,0.19,0.422,0.422,0.422h10.125c0.231,0,0.422-0.189,0.422-0.422v-3.797h3.797c0.232,0,0.422-0.19,0.422-0.422V2.828C17.812,2.596,17.623,2.406,17.391,2.406 M12.749,16.75h-9.28V7.469h3.375v5.484c0,0.231,0.19,0.422,0.422,0.422h5.483V16.75zM16.969,12.531H7.688V3.25h9.281V12.531z",
                    }
                }
            }
        }
    }
}
