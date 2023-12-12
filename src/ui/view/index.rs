use dioxus::prelude::*;

use crate::ui::{
    component::{Layout, Navbar},
    widget::{Drawer, DrawerButton, WidgetDataTransfer, WidgetDragState, WidgetManager},
};

pub fn Index(cx: Scope) -> Element {
    let drawer_toggle = use_state(cx, || false);
    let drawer_data_transfer = use_state(cx, || None::<WidgetDataTransfer>);
    let wm_drag = WidgetDragState::new(cx, false, None);

    render! {
        Drawer {
            toggle: drawer_toggle.clone(),
            data_transfer: drawer_data_transfer.clone(),
            drag: wm_drag.drag(),
            Layout {
                navbar: render! {
                    Navbar {
                        end: render! {
                            DrawerButton {
                                toggle: drawer_toggle.clone(),
                                drag_state: wm_drag.clone(),
                            }
                        }
                    }
                },
                WidgetManager { drag_state: wm_drag, data_transfer: drawer_data_transfer.clone() }
            }
        }
    }
}
