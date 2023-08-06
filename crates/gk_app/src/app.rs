use crate::event::{EventListener, EventMap};
use crate::handlers::{EventHandlerFn, EventHandlerFnOnce, UpdateHandlerFn};
use crate::storage::Storage;
use crate::window::GKWindowId;
use crate::{event, GKState};
use std::any::TypeId;

/// The core of the application, all the systems and backend interacts with it somehow
pub struct App<S: GKState + 'static> {
    pub(crate) storage: Storage<S>,
    pub(crate) event_handler: EventMap,
    pub(crate) initialized: bool,
    pub(crate) in_frame: bool,
    pub(crate) closed: bool,
}

impl<S: GKState> App<S> {
    /// Allows mutable access to a plugin stored
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.plugins.get_mut()
    }

    /// It's called when the backend is ready
    /// it dispatched the event `Init`
    pub fn init(&mut self) {
        gk_profile::function!();
        if self.initialized {
            return;
        }

        self.initialized = true;
        self.event(event::Init);
    }

    pub fn frame_start(&mut self) {
        gk_profile::tick!();
        gk_profile::function!();

        if self.in_frame {
            return;
        }

        self.in_frame = true;
        self.event(event::FrameStart);
    }

    pub fn frame_end(&mut self) {
        gk_profile::function!();

        if !self.in_frame {
            return;
        }

        self.event(event::FrameEnd);
        self.in_frame = false;
    }

    /// Execute any listener set for the event passed in
    pub fn event<E: 'static>(&mut self, evt: E) {
        gk_profile::function!();

        if !self.initialized {
            return;
        }

        let list = self.event_handler.get_mut(&TypeId::of::<E>());
        if let Some(list) = list {
            let mut needs_clean = false;
            list.iter_mut().for_each(|listener| match listener {
                EventListener::Once(_, cb_opt) => {
                    if let Some(cb) = cb_opt.take() {
                        let cb = cb.downcast::<Box<EventHandlerFnOnce<E, S>>>();
                        if let Ok(cb) = cb {
                            cb(&mut self.storage, &evt);
                            needs_clean = true;
                        }
                    }
                }
                EventListener::Mut(_, cb) => {
                    let cb = cb.downcast_mut::<Box<EventHandlerFn<E, S>>>();
                    if let Some(cb) = cb {
                        cb(&mut self.storage, &evt);
                    }
                }
            });

            // clean "once" listeners
            if needs_clean {
                list.retain(|listener| !listener.is_once());
            }
        }

        if !self.closed {
            execute_queued_events(self);
        }
    }

    /// It's called each frame by the backend and it dispatches
    /// the event `Update`.
    pub fn update(&mut self) {
        gk_profile::function!();

        if !(self.initialized && self.in_frame) {
            return;
        }

        self.event(event::Update);
    }

    pub fn draw(&mut self, window_id: GKWindowId) {
        gk_profile::function!();

        if !(self.initialized && self.in_frame) {
            return;
        }

        self.event(event::Draw { window_id });
    }

    /// It's called when the backend/app is about to close
    /// it dispatched the events `RequestedClose` and `Close`
    pub fn close(&mut self) {
        gk_profile::function!();

        if !self.initialized {
            return;
        }

        if self.closed {
            return;
        }

        self.event(event::RequestedClose);
        self.closed = true;
        self.event(event::Close);
    }
}

#[inline]
fn execute_queued_events<S: GKState + 'static>(app: &mut App<S>) {
    while let Some(cb) = app.storage.take_event() {
        cb(app);
    }
}
