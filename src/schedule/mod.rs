mod event;

use chrono::prelude::*;
use std::rc::Rc;

pub use event::{BasicEvent, Event};

#[derive(Default)]
pub struct Schedule {
    events: Vec<Rc<dyn Event>>,
}

impl Schedule {
    pub fn new(events: Vec<Rc<dyn Event>>) -> Self {
        Self { events }
    }

    pub fn events(&self) -> &Vec<Rc<dyn Event>> {
        &self.events
    }

    pub fn events_on_date<'a>(
        &'a self,
        date: &'a NaiveDate,
    ) -> impl Iterator<Item = Rc<dyn Event + 'a>> {
        self.events
            .iter()
            .filter_map(|e| (e.datetime().date() == *date).then_some(e.clone()))
    }
}
