use crate::ql::Predicate;
use chrono::prelude::*;
use std::{
    cell::{Cell, RefCell},
    fmt,
    rc::Rc,
};

#[derive(Copy, Clone)]
pub struct Time(pub NaiveTime);

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0.hour(), self.0.minute())
    }
}

#[derive(Copy, Clone)]
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
