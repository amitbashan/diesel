use std::rc::Rc;

use chrono::{prelude::*, Duration};

pub trait Event {
    fn title(&self) -> Rc<String>;
    fn description(&self) -> Option<Rc<String>> {
        None
    }
    fn datetime(&self) -> NaiveDateTime;
    fn duration(&self) -> Duration;
}

pub struct BasicEvent {
    pub title: Rc<String>,
    pub description: Rc<String>,
    pub date: NaiveDateTime,
    pub duration: Duration,
}

impl Event for BasicEvent {
    fn title(&self) -> Rc<String> {
        self.title.clone()
    }

    fn description(&self) -> Option<Rc<String>> {
        Some(self.description.clone())
    }

    fn datetime(&self) -> NaiveDateTime {
        self.date
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}
