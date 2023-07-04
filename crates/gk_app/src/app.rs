use crate::handlers::{EventHandler, EventHandlerFn, UpdateHandlerFn};
use crate::storage::Storage;
use crate::GKState;
use gk_core::events::Event;
use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

pub struct App<S: GKState + 'static> {
    pub(crate) storage: Storage<S>,
    pub(crate) init_handler: Box<UpdateHandlerFn<S>>,
    pub(crate) event_handler: HashMap<TypeId, Box<dyn Any>>,
    pub(crate) update_handler: Box<UpdateHandlerFn<S>>,
    pub(crate) close_handler: Box<UpdateHandlerFn<S>>,
    pub(crate) initialized: bool,
    pub(crate) closed: bool,
}

impl<S: GKState> App<S> {
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.plugins.get_mut()
    }

    pub fn init(&mut self) {
        if self.initialized {
            return;
        }

        self.initialized = true;
        (self.init_handler)(&mut self.storage);
    }

    pub fn event<E: 'static>(&mut self, evt: E) {
        let opt_cb = self
            .event_handler
            .get_mut(&TypeId::of::<E>())
            .and_then(|cb| cb.downcast_mut::<Box<EventHandlerFn<E, S>>>());

        if let Some(cb) = opt_cb {
            cb(&mut self.storage, evt);
        }

        execute_queued_events(self);
    }

    pub fn update(&mut self) {
        (self.update_handler)(&mut self.storage);
    }

    pub fn close(&mut self) {
        if self.closed {
            return;
        }

        self.closed = true;
        (self.close_handler)(&mut self.storage);
    }
}

#[inline]
fn execute_queued_events<S: GKState + 'static>(app: &mut App<S>) {
    while let Some(cb) = app.storage.take_event() {
        cb(app);
    }
}
