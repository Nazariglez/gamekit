use crate::handlers::{CustomEventHandler, CustomEventHandlerFn, EventHandlerFn, UpdateHandlerFn};
use crate::storage::Storage;
use crate::GKState;
use gk_core::events::{Event, EventIterator};
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct App<S: GKState + 'static> {
    pub(crate) storage: Storage<S>,
    pub(crate) events: EventIterator,
    pub(crate) event_handler: Box<EventHandlerFn<S>>,
    pub(crate) ee_handler: HashMap<TypeId, Box<dyn Any>>,
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
        (self.event_handler)(&mut self.storage, evt);
    }

    pub fn custom_event<E: 'static>(&mut self, evt: E) {
        let k = TypeId::of::<E>();
        let opt_cb = self
            .ee_handler
            .get_mut(&TypeId::of::<E>())
            .and_then(|cb| cb.downcast_mut::<Box<CustomEventHandlerFn<E, S>>>());
        if let Some(cb) = opt_cb {
            cb(&mut self.storage, evt);
        }
    }

    pub fn tick(&mut self) {
        (self.update_handler)(&mut self.storage);
    }

    pub fn close(&mut self) {
        // TODO execute when app is closed
    }
}
