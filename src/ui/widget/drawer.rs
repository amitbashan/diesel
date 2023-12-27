use dioxus::prelude::*;
use strum::IntoEnumIterator;

use super::{Widget, WidgetDataTransfer, WidgetDragState, WidgetManagerState};
use crate::ui::{component::svg, widget::WidgetStates};

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
    let widget_states = use_shared_state::<WidgetStates>(cx)?;
    let make_preview_container = |widget: Widget| {
        let w = widget.component();
        let sizes = widget.sizes();
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
                        w(cx, size, widget_states)
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
                    class: "carousel carousel-center w-full max-w-xs items-center p-4 space-x-4 bg-neutral rounded-box",
                    containers
                },
            }
        }
    };
    let containers = Widget::iter().map(|w| make_preview_container(w));

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
                    containers
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
    const CLASS: &str = "btn btn-neutral";
    let wms = use_shared_state::<WidgetManagerState>(cx)?;

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
                svg::Trash {}
            }
        }
    } else {
        render! {
            button {
                class: CLASS,
                onclick: move |_| {
                    toggle.modify(|t| !t);
                },
                svg::StackedWindows {}
            }
        }
    }
}
