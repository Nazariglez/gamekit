use crate::event::EventQueue;
use crate::{App, GKState, Plugin};
use anymap::AnyMap;
use gk_core::events::Event;
use indexmap::IndexMap;

pub struct Storage<S: GKState + 'static> {
    pub state: S,
    pub plugins: Plugins,
    pub events: EventQueue<S>,
}

impl<S: GKState + 'static> Storage<S> {
    pub fn take_event(&mut self) -> Option<Box<dyn FnOnce(&mut App<S>)>> {
        self.events.take_event()
    }
}

pub struct Plugins {
    map: AnyMap,
}

impl Plugins {
    pub(crate) fn new() -> Self {
        Self { map: AnyMap::new() }
    }

    pub(crate) fn add<T: 'static>(&mut self, plugin: T) {
        self.map.insert(plugin);
    }

    pub(crate) fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut()
    }
}

pub trait FromPlugins {
    fn from_plugins(storage: &mut Plugins) -> &mut Self;
}

impl<T: 'static> FromPlugins for T {
    fn from_plugins(storage: &mut Plugins) -> &mut Self {
        storage.map.get_mut::<Self>().unwrap()
    }
}

pub trait FromStorage<S: GKState> {
    fn from_storage<'gk_state>(app: &'gk_state mut Storage<S>) -> &'gk_state mut Self;
}

impl<S: GKState, T: Plugin + 'static> FromStorage<S> for T {
    fn from_storage(storage: &mut Storage<S>) -> &mut Self {
        storage.plugins.map.get_mut::<Self>().unwrap()
    }
}

impl<S: GKState + 'static> FromStorage<S> for EventQueue<S> {
    fn from_storage(storage: &mut Storage<S>) -> &mut Self {
        &mut storage.events
    }
}
//
// pub trait FromStorageEvent<S: GKState> {
//     fn from_storage<'gk_state>(app: &'gk_state mut Storage<S>, event: Event) -> &'gk_state mut Self;
// }
//
// impl<S: GKState, T: Plugin + 'static> FromStorageEvent<S> for T {
//     fn from_storage(storage: &mut Storage<S>, _event: Event) -> &mut Self {
//         storage.plugins.map.get_mut::<Self>().unwrap()
//     }
// }
//
// impl<S: GKState> FromStorageEvent<S> for Event {
//     fn from_storage(storage: &mut Storage<S>, event: Event) -> &mut Self {
//
//     }
// }
