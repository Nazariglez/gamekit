use crate::handlers::EventHandlerFn;
use crate::storage::Storage;
use crate::GKState;

pub struct App<S: GKState> {
    pub(crate) storage: Storage<S>,
    pub(crate) events: Vec<()>,
    pub(crate) event_handler: Box<EventHandlerFn<S>>,
    pub(crate) initialized: bool,
}

impl<S: GKState> App<S> {
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.plugins.get_mut()
    }

    pub fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        self.initialized = true;
        // TODO execute init callback
    }

    pub fn tick(&mut self) {
        (self.event_handler)(&mut self.storage);
    }
}
