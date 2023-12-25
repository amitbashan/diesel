use std::{borrow::Cow, collections::HashMap, fs, io, path::Path, rc::Rc};

use crate::{
    ql::grammar,
    schedule,
    ui::{
        widget::{WidgetManagerState, WidgetStates},
        Theme,
    },
};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub title: String,
    pub description: String,
    pub predicate: String,
    pub time_pair: String,
}

impl Event {
    pub fn parse(self) -> Option<schedule::event::Event> {
        let predicate = grammar::PredicateParser::new()
            .parse(&self.predicate)
            .ok()?;
        let time_pair = grammar::TimePairParser::new().parse(&self.time_pair).ok()?;

        Some(schedule::event::Event::new(
            self.title,
            self.description,
            predicate,
            time_pair,
        ))
    }
}

impl From<&schedule::Event> for Event {
    fn from(value: &schedule::Event) -> Self {
        let title = value.title().as_ref().borrow().clone();
        let description = value.description().as_ref().borrow().clone();
        let predicate = value.predicate().get().to_string();
        let time_pair = value.time_pair().get().to_string();
        Self {
            title,
            description,
            predicate,
            time_pair,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Schedule {
    events: Vec<Event>,
}

impl Schedule {
    pub fn parse(self) -> schedule::Schedule {
        let events = self
            .events
            .into_iter()
            .filter_map(|e| e.parse().map(|e| Rc::new(e)))
            .collect();
        schedule::Schedule::new(events)
    }
}

impl From<&schedule::Schedule> for Schedule {
    fn from(value: &schedule::Schedule) -> Self {
        let events = value.events().iter().map(|e| e.as_ref().into()).collect();
        Self { events }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Configuration {
    theme: String,
    widget_manager_state: WidgetManagerState,
    widget_states: HashMap<String, HashMap<String, String>>,
    schedule: Schedule,
}

impl Configuration {
    pub fn import<P: AsRef<Path>>(path: P) -> io::Result<Option<Self>> {
        let str = fs::read_to_string(path)?;
        let cfg = serde_json::from_str(&str).ok();
        Ok(cfg)
    }

    pub fn load(self, cx: &ScopeState) {
        let theme = use_shared_state::<Theme>(cx).expect("theme state not initialized");
        let widget_manager_state = use_shared_state::<WidgetManagerState>(cx)
            .expect("widget manager state not initialized");
        let widget_states =
            use_shared_state::<WidgetStates>(cx).expect("widget states not initialized");
        let schedule =
            use_shared_state::<schedule::Schedule>(cx).expect("schedule not initialized");

        theme.with_mut(|t| t.0 = Cow::Owned(self.theme));
        widget_manager_state.with_mut(|s| *s = self.widget_manager_state);
        widget_states.with_mut(|s| s.0 = self.widget_states);
        schedule.with_mut(|s| *s = self.schedule.parse());
    }

    pub fn save(cx: &ScopeState) -> Self {
        Self::try_save(cx).expect("failed to save configuration")
    }

    pub fn try_save(cx: &ScopeState) -> io::Result<Self> {
        let theme = use_shared_state::<Theme>(cx).expect("theme state not initialized");
        let widget_manager_state = use_shared_state::<WidgetManagerState>(cx)
            .expect("widget manager state not initialized");
        let widget_states =
            use_shared_state::<WidgetStates>(cx).expect("widget states not initialized");
        let schedule =
            use_shared_state::<schedule::Schedule>(cx).expect("schedule not initialized");
        let cfg_path = use_shared_state::<ConfigurationPath>(cx)
            .expect("configuration path state not initialized");
        let cfg_path = cfg_path.read();
        let cfg_path = cfg_path.0.as_ref().expect("configuration path not set");
        let cfg = Self {
            theme: theme.with(|t| t.0.to_string()),
            widget_manager_state: widget_manager_state.with(|s| s.clone()),
            widget_states: widget_states.with(|s| s.0.clone()),
            schedule: schedule.with(|s| s.into()),
        };
        let vec = serde_json::to_vec(&cfg).expect("failed to serialize configuration");
        fs::write(cfg_path, &vec)?;
        Ok(cfg)
    }
}

#[derive(Default)]
pub struct ConfigurationPath(pub Option<String>);
