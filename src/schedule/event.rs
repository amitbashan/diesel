use crate::ql::Predicate;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::{Cell, RefCell},
    fmt,
    rc::Rc,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct Time(pub NaiveTime);

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

pub struct Event {
    title: Rc<RefCell<String>>,
    description: Rc<RefCell<String>>,
    predicate: Rc<RefCell<Predicate>>,
    time_pair: Rc<Cell<TimePair>>,
}

impl Event {
    pub fn new(
        title: String,
        description: String,
        predicate: Predicate,
        time_pair: TimePair,
    ) -> Self {
        Self {
            title: Rc::new(RefCell::new(title)),
            description: Rc::new(RefCell::new(description)),
            predicate: Rc::new(RefCell::new(predicate)),
            time_pair: Rc::new(Cell::new(time_pair)),
        }
    }

    pub fn title(&self) -> Rc<RefCell<String>> {
        self.title.clone()
    }

    pub fn description(&self) -> Rc<RefCell<String>> {
        self.description.clone()
    }

    pub fn predicate(&self) -> Rc<RefCell<Predicate>> {
        self.predicate.clone()
    }

    pub fn time_pair(&self) -> Rc<Cell<TimePair>> {
        self.time_pair.clone()
    }
}
