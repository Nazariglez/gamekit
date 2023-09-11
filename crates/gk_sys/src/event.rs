use crate::window::GKWindowId;
use crate::{option_usize_env, GKState, System};
use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

#[cfg(feature = "limited_events")]
const MAX_EVENT_LISTENERS: usize = option_usize_env!("GK_LIMIT_EVENTS_TO", 32);

#[cfg(feature = "limited_events")]
pub(crate) type EventMap = HashMap<TypeId, arrayvec::ArrayVec<EventListener, MAX_EVENT_LISTENERS>>;

#[cfg(not(feature = "limited_events"))]
pub(crate) type EventMap = HashMap<TypeId, Vec<EventListener>>;

pub(crate) enum EventListener {
    Once(u64, Option<Box<dyn Any>>),
    Mut(u64, Box<dyn Any>),
}

impl EventListener {
    pub(crate) fn is_once(&self) -> bool {
        if let Self::Once(_, _) = self {
            return true;
        }

        false
    }
}

/// A list of events pushed by plugins to be processed
#[derive(Default)]
pub struct EventQueue<S: GKState + 'static> {
    pub(crate) events: VecDeque<Box<dyn FnOnce(&mut System<S>)>>,
}

impl<S: GKState + 'static> EventQueue<S> {
    pub(crate) fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    /// Add a new event to the queue
    pub fn queue<E: Send + Sync + std::fmt::Debug + 'static>(&mut self, event: E) {
        self.events.push_back(Box::new(move |app| app.event(event)));
    }

    /// Take the first event of the queue
    pub(crate) fn take_event(&mut self) -> Option<Box<dyn FnOnce(&mut System<S>)>> {
        self.events.pop_front()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Init;

#[derive(Debug, Copy, Clone)]
pub struct FrameStart;

#[derive(Debug, Copy, Clone)]
pub struct Update;

#[derive(Debug, Copy, Clone)]
pub struct DrawRequest {
    pub window_id: GKWindowId,
}

#[derive(Debug, Copy, Clone)]
pub struct FrameEnd;

#[derive(Debug, Copy, Clone)]
pub struct RequestedClose;

#[derive(Debug, Copy, Clone)]
pub struct Close;
