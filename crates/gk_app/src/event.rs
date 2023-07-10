use crate::{option_usize_env, App, GKState};
use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

#[cfg(feature = "limited_events")]
const MAX_EVENT_LISTENERS: usize = option_usize_env!("GK_LIMIT_EVENTS_TO", 32);

#[cfg(feature = "limited_events")]
pub(crate) type EventMap = HashMap<TypeId, arrayvec::ArrayVec<Box<dyn Any>, MAX_EVENT_LISTENERS>>;

#[cfg(not(feature = "limited_events"))]
pub(crate) type EventMap = HashMap<TypeId, Vec<Box<dyn Any>>>;

/// A list of events pushed by plugins to be processed
#[derive(Default)]
pub struct EventQueue<S: GKState + 'static> {
    pub(crate) events: VecDeque<Box<dyn FnOnce(&mut App<S>)>>,
}

impl<S: GKState + 'static> EventQueue<S> {
    pub(crate) fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    /// Add a new event to the queue
    pub fn queue<E: 'static>(&mut self, event: E) {
        self.events.push_back(Box::new(move |app| app.event(event)));
    }

    /// Take the first event of the queue
    pub(crate) fn take_event(&mut self) -> Option<Box<dyn FnOnce(&mut App<S>)>> {
        self.events.pop_front()
    }
}

/// Triggered before the user's initialize callback
#[derive(Debug, Copy, Clone)]
pub struct Init;

/// First event triggered per frame
#[derive(Debug, Copy, Clone)]
pub struct PreUpdate;

/// Triggered between pre and post update events (before user's update callback)
#[derive(Debug, Copy, Clone)]
pub struct Update;

/// Latest event triggered per frame
#[derive(Debug, Copy, Clone)]
pub struct PostUpdate;

/// Triggered after user's close callback
#[derive(Debug, Copy, Clone)]
pub struct Close;
