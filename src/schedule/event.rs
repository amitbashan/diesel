use crate::ql::Predicate;
use chrono::Duration;
use std::rc::Rc;

pub type Time = (u8, u8);

pub type TimeSpan = (Time, Duration);

pub trait Event {
    fn title(&self) -> &Rc<String>;
    fn description(&self) -> Option<&Rc<String>> {
        None
    }
    fn predicate(&self) -> &Predicate;
    fn time_span(&self) -> TimeSpan;
}

pub struct SkeletonEvent {
    pub title: Rc<String>,
    pub description: Rc<String>,
    predicate: Predicate,
    time: Time,
    duration: Duration,
}

impl SkeletonEvent {
    pub fn new(
        title: String,
        description: String,
        predicate: Predicate,
        time: Time,
        duration: Duration,
    ) -> Self {
        Self {
            title: Rc::new(title),
            description: Rc::new(description),
            predicate,
            time,
            duration,
        }
    }
}

impl Event for SkeletonEvent {
    fn title(&self) -> &Rc<String> {
        &self.title
    }

    fn description(&self) -> Option<&Rc<String>> {
        Some(&self.description)
    }

    fn predicate(&self) -> &Predicate {
        &self.predicate
    }

    fn time_span(&self) -> TimeSpan {
        (self.time, self.duration)
    }
}
