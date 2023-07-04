use crate::{App, GKState};
use std::collections::VecDeque;

#[derive(Default)]
pub struct EventQueue<S: GKState + 'static> {
    pub(crate) events: VecDeque<Box<dyn FnOnce(&mut App<S>)>>,
}

impl<S: GKState + 'static> EventQueue<S> {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn queue<E: 'static>(&mut self, event: E) {
        self.events.push_back(Box::new(move |app| app.event(event)));
    }

    pub fn take_event(&mut self) -> Option<Box<dyn FnOnce(&mut App<S>)>> {
        self.events.pop_front()
    }
}
