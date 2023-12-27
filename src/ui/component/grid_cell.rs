use dioxus::prelude::*;

use crate::ui::widget::{
    WidgetDataTransfer, WidgetDragState, WidgetManagerState, WidgetStates, WIDGETS,
};

#[component]
pub fn GridCell<'a>(
    cx: Scope<'a>,
    class: Option<&'a str>,
    data_transfer: UseState<Option<WidgetDataTransfer>>,
    drag_state: WidgetDragState<'a>,
    #[props(!optional)] widget_data: Option<WidgetDataTransfer>,
    cell_index: usize,
) -> Element {
    const DRAG_OPACITY: u8 = 25;
    let class = class.unwrap_or_default();
    let wms = use_shared_state::<WidgetManagerState>(cx)?;
    let widget_states = use_shared_state::<WidgetStates>(cx)?;

    if let Some(mut wdt) = widget_data {
        let widget = WIDGETS[wdt.widget as usize];
        let sc = wdt.size.class();
        render! {
            div {
                class: "flex flex-1 h-full rounded-lg shadow-xl max-h-full overflow-hidden {sc} {class}",
                draggable: true,
                prevent_default: "ondragover",
                ondragover: move |e| {
                    e.stop_propagation();
                },
                ondragstart: move |_| {
                    let i = Some(*cell_index);
                    wdt.source_index = i;
                    data_transfer.set(Some(wdt));
                    drag_state.drag.set(true);
                    drag_state.cell_index.set(i);
                },
                ondragend: move |_| {
                    data_transfer.set(None);
                    drag_state.drag.set(false);
                },
                widget(cx, wdt.size, widget_states),
            }
        }
    } else {
        let opacity = drag_state.drag.then_some(DRAG_OPACITY).unwrap_or_default();
        render! {
            div {
                class: "flex h-full rounded-lg shadow-xl bg-neutral {class}",
                opacity: "{opacity}%",
                transition: "opacity 0.3s",
                prevent_default: "ondragover",
                ondragover: move |e| {
                    e.stop_propagation();
                },
                ondrop: move |_| {
                    if let Some(d) = data_transfer.get() {
                        if let Some(s) = d.source_index {
                            wms.with_mut(|wms| wms.move_widget(*cell_index, &s));
                        } else {
                            wms.with_mut(|wms| wms.add_widget(*cell_index, *d));
                        }
                        data_transfer.set(None);
                    }
                    drag_state.drag.set(false);
                },
            }
        }
    }
}
