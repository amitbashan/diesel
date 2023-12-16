mod event;

use std::rc::Rc;

pub use event::SkeletonEvent;

use chrono::prelude::*;
pub use event::Event;

use crate::ql::{Context, Evaluate};

#[derive(Default)]
pub struct Schedule {
    events: Vec<Rc<dyn Event>>,
}

impl Schedule {
    pub fn schedule_event(&mut self, event: Rc<dyn Event>) {
        self.events.push(event);
    }

    pub fn get_events_on_date(&self, date: NaiveDate) -> impl Iterator<Item = &Rc<dyn Event>> {
        let context = Context { date };
        self.events
            .iter()
            .filter(move |event| event.predicate().evaluate(&context))
    }
}
