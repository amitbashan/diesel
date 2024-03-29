use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod drawer;
mod size;
mod time;
mod upcoming;
pub mod weather;

pub use drawer::{Drawer, DrawerButton};
use strum::EnumIter;
pub use time::TimeWidget;

use crate::ui::component::GridCell;

pub use size::WidgetSize;

use self::{
    upcoming::UpcomingEventsWidget,
    weather::{WeatherWidget, WeatherWidgetState},
};

type WidgetComponent =
    for<'a> fn(&'a ScopeState, WidgetSize, &'a UseSharedState<WidgetStates>) -> Element<'a>;

pub const WIDGETS: [WidgetComponent; 3] = [TimeWidget, WeatherWidget, UpcomingEventsWidget];
const ROWS: usize = 4;
const COLS: usize = 7;

#[derive(Debug, Copy, Clone, EnumIter, Serialize, Deserialize)]
pub enum Widget {
    Time,
    Weather,
    UpcomingEvents,
}

impl Widget {
    pub fn sizes(&self) -> [bool; 3] {
        match self {
            Self::Time => [true, true, true],
            Self::Weather => [true, true, false],
            Self::UpcomingEvents => [false, true, true],
        }
    }

    pub fn component(&self) -> WidgetComponent {
        WIDGETS[*self as usize]
    }
}

impl fmt::Display for Widget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Time => "Time (debug widget)",
                Self::Weather => "Weather",
                Self::UpcomingEvents => "Upcoming Events",
            }
        )
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct WidgetDataTransfer {
    pub widget: Widget,
    pub size: WidgetSize,
    #[serde(skip)]
    pub source_index: Option<usize>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct WidgetManagerState {
    cell_to_widget_data: HashMap<usize, WidgetDataTransfer>,
}

impl WidgetManagerState {
    pub fn add_widget(&mut self, i: usize, data: WidgetDataTransfer) -> Option<WidgetDataTransfer> {
        if self.is_within_bounds(i, data.size, None) {
            self.cell_to_widget_data.insert(i, data)
        } else {
            None
        }
    }

    pub fn remove_widget(&mut self, i: &usize) -> Option<WidgetDataTransfer> {
        self.cell_to_widget_data.remove(i)
    }

    pub fn move_widget(&mut self, dst: usize, src: &usize) {
        if let Some(size) = self.cell_to_widget_data.get(src).map(|d| d.size) {
            if self.is_within_bounds(dst, size, Some(*src)) {
                let data = self.remove_widget(src).unwrap();
                self.add_widget(dst, data);
            }
        }
    }

    pub fn occupied_cells<'a>(&'a self, with_root_cell: bool) -> impl Iterator<Item = usize> + 'a {
        self.cell_to_widget_data
            .iter()
            .flat_map(move |(k, v)| Self::calculate_occupying_indices(*k, v.size, with_root_cell))
    }

    fn calculate_occupying_indices(
        k: usize,
        size: WidgetSize,
        with_root_cell: bool,
    ) -> impl Iterator<Item = usize> {
        (0..size.row_span()).flat_map(move |i| {
            (0..size.col_span())
                .filter_map(move |j| (with_root_cell || i + j != 0).then_some(k + i * COLS + j))
        })
    }

    fn is_within_bounds(&self, dst: usize, size: WidgetSize, src: Option<usize>) -> bool {
        !self.is_colliding(dst, size, src) && Self::is_within_border(dst, size)
    }

    fn is_colliding(&self, i: usize, size: WidgetSize, src: Option<usize>) -> bool {
        let mut occupied: HashSet<_> = self.occupied_cells(true).collect();
        let mut to_be_occupied = Self::calculate_occupying_indices(i, size, true);
        if let Some(src) = src {
            let occupied_by_source: HashSet<_> =
                Self::calculate_occupying_indices(src, size, true).collect();
            occupied = occupied.difference(&occupied_by_source).copied().collect();
        }
        to_be_occupied.any(|j| occupied.contains(&j))
    }

    const fn is_within_border(i: usize, size: WidgetSize) -> bool {
        let isnt_within_right_border = i % COLS + size.col_span() - 1 == COLS;
        let isnt_within_bottom_border = i / COLS + size.row_span() - 1 == ROWS;
        !(isnt_within_right_border || isnt_within_bottom_border)
    }
}

#[derive(Clone, PartialEq)]
pub struct WidgetDragState<'a> {
    pub drag: &'a UseState<bool>,
    pub cell_index: &'a UseState<Option<usize>>,
}

impl<'a> WidgetDragState<'a> {
    pub fn new(cx: &'a ScopeState, drag: bool, cell_index: Option<usize>) -> Self {
        let drag = use_state(cx, || drag);
        let cell_index = use_state(cx, || cell_index);
        Self { drag, cell_index }
    }

    pub fn drag(&self) -> UseState<bool> {
        self.drag.clone()
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct WidgetStates {
    weather: WeatherWidgetState,
}

#[component]
pub fn WidgetManager<'a>(
    cx: Scope<'a>,
    drag_state: WidgetDragState<'a>,
    data_transfer: UseState<Option<WidgetDataTransfer>>,
) -> Element {
    let wms = use_shared_state::<WidgetManagerState>(cx)?.read();
    let occupied_cells: HashSet<_> = wms.occupied_cells(false).collect();
    let cells = (0..ROWS * COLS).filter_map(|i| {
        (!occupied_cells.contains(&i)).then_some({
            let wd = wms.cell_to_widget_data.get(&i).cloned();
            render! {
                GridCell {
                    drag_state: drag_state.clone(),
                    data_transfer: data_transfer.clone(),
                    widget_data: wd,
                    cell_index: i,
                }
            }
        })
    });

    render! {
        div {
            class: "grid grid-rows-4 grid-cols-7 p-2 gap-1 items-start rounded flex-1 min-h-0 bg-base-200",
            cells,
        }
    }
}
