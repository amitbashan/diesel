use crate::ql::Predicate;
use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};
use std::{
    cell::{Cell, RefCell},
    fmt,
    rc::Rc,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct Time(pub NaiveTime);

impl Time {
    pub fn as_duration(&self) -> Duration {
        let hour = self.0.hour();
        let minute = self.0.minute();
        Duration::hours(hour as i64) + Duration::minutes(minute as i64)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0.hour(), self.0.minute())
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct TimePair(pub Time, pub Time);

impl fmt::Display for TimePair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

pub trait Event {
    fn title(&self) -> Rc<RefCell<String>>;
    fn description(&self) -> Rc<RefCell<String>>;
    fn predicate(&self) -> Rc<Cell<Predicate>>;
    fn time_pair(&self) -> Rc<Cell<TimePair>>;
}

pub struct SkeletonEvent {
    pub title: Rc<RefCell<String>>,
    pub description: Rc<RefCell<String>>,
    predicate: Rc<Cell<Predicate>>,
    time_pair: Rc<Cell<TimePair>>,
}

impl SkeletonEvent {
    pub fn new(
        title: String,
        description: String,
        predicate: Predicate,
        time_pair: TimePair,
    ) -> Self {
        Self {
            title: Rc::new(RefCell::new(title)),
            description: Rc::new(RefCell::new(description)),
            predicate: Rc::new(Cell::new(predicate)),
            time_pair: Rc::new(Cell::new(time_pair)),
        }
    }
}

impl Event for SkeletonEvent {
    fn title(&self) -> Rc<RefCell<String>> {
        self.title.clone()
    }

    fn description(&self) -> Rc<RefCell<String>> {
        self.description.clone()
    }

    fn predicate(&self) -> Rc<Cell<Predicate>> {
        self.predicate.clone()
    }

    fn time_pair(&self) -> Rc<Cell<TimePair>> {
        self.time_pair.clone()
    }
}
