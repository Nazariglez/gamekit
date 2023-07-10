use crate::event::{AppEvent, EventMap};
use crate::handlers::{EventHandlerFn, UpdateHandlerFn};
use crate::storage::Storage;
use crate::GKState;
use std::any::TypeId;

/// The core of the application, all the systems and backend interacts with it somehow
pub struct App<S: GKState + 'static> {
    pub(crate) storage: Storage<S>,
    pub(crate) init_handler: Box<UpdateHandlerFn<S>>,
    pub(crate) event_handler: EventMap,
    pub(crate) update_handler: Box<UpdateHandlerFn<S>>,
    pub(crate) close_handler: Box<UpdateHandlerFn<S>>,
    pub(crate) initialized: bool,
    pub(crate) closed: bool,
}

impl<S: GKState> App<S> {
    /// Allows mutable access to a plugin stored
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.plugins.get_mut()
    }

    /// It's called when the backend is ready
    /// it dispartched the event `Init`
    pub fn init(&mut self) {
        if self.initialized {
            return;
        }

        self.event(AppEvent::Init);

        self.initialized = true;
        (self.init_handler)(&mut self.storage);
    }

    /// Execute any listener set for the event passed in
    pub fn event<E: 'static>(&mut self, evt: E) {
        let list = self.event_handler.get_mut(&TypeId::of::<E>());
        if let Some(list) = list {
            list.iter_mut()
                .filter_map(|cb| cb.downcast_mut::<Box<EventHandlerFn<E, S>>>())
                .for_each(|cb| {
                    cb(&mut self.storage, &evt);
                    // execute_queued_events(self);
                });
        }

        execute_queued_events(self);
    }

    /// It's called each frame by the backend and it dispatches
    /// the events `PreUpdate`, `Update` and `PostUpdate`
    pub fn update(&mut self) {
        self.event(AppEvent::PreUpdate);
        self.event(AppEvent::Update);
        (self.update_handler)(&mut self.storage);
        self.event(AppEvent::PostUpdate);
    }

    /// It's called when the backend/app is about to close
    /// it dispatched the event `Close`
    pub fn close(&mut self) {
        if self.closed {
            return;
        }

        self.closed = true;
        (self.close_handler)(&mut self.storage);
        self.event(AppEvent::Close);
    }
}

#[inline]
fn execute_queued_events<S: GKState + 'static>(app: &mut App<S>) {
    while let Some(cb) = app.storage.take_event() {
        cb(app);
    }
}
