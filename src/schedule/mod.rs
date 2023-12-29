pub mod event;

use std::rc::Rc;

pub use event::Event;

pub use event::TimePair;

use crate::ql::{Context, Evaluate, Predicate};

#[derive(Default)]
pub struct Schedule {
    events: Vec<Rc<Event>>,
}

impl Schedule {
    pub fn new(events: Vec<Rc<Event>>) -> Self {
        Self { events }
    }

    pub fn events(&self) -> &Vec<Rc<Event>> {
        &self.events
    }

    pub fn schedule_event(&mut self, event: Rc<Event>) {
        self.events.push(event);
    }

    pub fn cancel_event(&mut self, i: usize) -> Option<Rc<Event>> {
        (self.events.len() > i).then_some(self.events.remove(i))
    }

    pub fn edit_event(
        &mut self,
        i: usize,
        new_title: Option<String>,
        new_description: Option<String>,
        new_predicate: Option<Predicate>,
        new_time_pair: Option<TimePair>,
    ) {
        if let Some(event) = self.events.get(i) {
            if let Some(new_title) = new_title {
                let title = event.title();
                title.replace(new_title);
            }

            if let Some(new_description) = new_description {
                let description = event.description();
                description.replace(new_description);
            }

            if let Some(new_predicate) = new_predicate {
                let predicate = event.predicate();
                predicate.replace(new_predicate);
            }

            if let Some(new_time_pair) = new_time_pair {
                let time_pair = event.time_pair();
                time_pair.replace(new_time_pair);
            }
        }
    }

    pub fn get_event(&self, i: usize) -> Option<&Rc<Event>> {
        self.events.get(i)
    }

    pub fn get_events_on_date(
        &self,
        context: Context,
    ) -> impl Iterator<Item = (usize, &Rc<Event>)> {
        self.events.iter().enumerate().filter(move |(_, event)| {
            let predicate = event.predicate();
            let predicate = predicate.borrow();
            predicate.evaluate(&context).ok().unwrap_or_default()
        })
    }
}
