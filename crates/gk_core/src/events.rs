use std::collections::VecDeque;

use crate::window::GKWindowId;

pub struct SuperEvent;

/// Window's event
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowEvent {
    /// Window's position after it was moved
    Moved { x: i32, y: i32 },

    /// Window's size after it was resized
    Resized { width: u32, height: i32 },

    /// The window was minimized
    Minimized,

    /// The window was maximized
    Maximized,

    /// The window did gain the focus
    FocusGained,

    /// The window did lost the focus
    FocusLost,

    /// The window has received the close signal
    CloseRequest,
}

/// Application events usually received from the user
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Event {
    /// When the app is about to close
    Close,

    WindowEvent {
        id: GKWindowId,
        event: WindowEvent,
    },
}

/// Event iterator queue
#[derive(Debug, Clone, Default)]
pub struct EventIterator(VecDeque<Event>);

impl EventIterator {
    /// Remove and return the first element on the queue
    pub fn pop_front(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    /// Add an event at the end of the list
    pub fn push(&mut self, evt: Event) {
        self.0.push_back(evt);
    }

    /// Add an event at the beginning of the list
    pub fn push_front(&mut self, evt: Event) {
        self.0.push_front(evt);
    }

    /// Return the events and clear the list
    pub fn take(&mut self) -> EventIterator {
        EventIterator(std::mem::take(&mut self.0))
    }
}

impl Iterator for EventIterator {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}
