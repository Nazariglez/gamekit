use crate::handlers::UpdateHandlerFn;
use crate::storage::Storage;
use crate::GKState;
use gk_core::events::{Event, EventIterator};

pub struct App<S: GKState> {
    pub(crate) storage: Storage<S>,
    pub(crate) events: EventIterator,
    pub(crate) update_handler: Box<UpdateHandlerFn<S>>,
    pub(crate) initialized: bool,
}

impl<S: GKState> App<S> {
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.plugins.get_mut()
    }

    pub fn add_event(&mut self, evt: Event) {
        self.events.push(evt);
    }

    pub fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        self.initialized = true;
        // TODO execute init callback
    }

    pub fn event(&mut self, evt: Event) {
        // todo
    }

    pub fn tick(&mut self) {
        (self.update_handler)(&mut self.storage);
    }

    pub fn close(&mut self) {
        // TODO execute when app is closed
    }
}
