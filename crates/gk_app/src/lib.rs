mod config;

use gk_core::{GKWindow, GKWindowId, GKWindowManager};
use std::marker::PhantomData;
use std::ops::Rem;

type RunnerHandlerFn<S> = dyn FnMut(App<S>) -> Result<(), String>;
type EventHandlerFn<S> = dyn FnMut(&mut Storage<S>);
type SetupHandlerFn<S> = dyn FnOnce(&mut Plugins) -> Result<S, String>;

pub struct App<S: GKState> {
    storage: Storage<S>,
    events: Vec<()>,
    event_handler: Box<EventHandlerFn<S>>,
}

impl<S: GKState> App<S> {
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.plugins.get_mut()
    }

    pub fn tick(&mut self) {
        (self.event_handler)(&mut self.storage);
    }
}

pub struct AppBuilder<S: GKState + 'static> {
    plugins: Plugins,
    runner: Box<RunnerHandlerFn<S>>,
    setup_handler: Box<SetupHandlerFn<S>>,
    event_handler: Box<EventHandlerFn<S>>,
    late_configs: Option<IndexMap<std::any::TypeId, Box<dyn BuildConfig<S>>>>,
}

impl GKState for () {}

impl AppBuilder<()> {
    pub fn init() -> Self {
        Self::init_with(|| Ok(()))
    }
}

impl<S: GKState> AppBuilder<S> {
    pub fn init_with<T, H>(handler: H) -> Self
    where
        H: SetupHandler<S, T> + 'static,
    {
        let mut plugins = Plugins::new();
        let runner = Box::new(default_runner);
        let setup_handler: Box<SetupHandlerFn<S>> = Box::new(|plugins| handler.call(plugins));
        let event_handler: Box<EventHandlerFn<S>> = Box::new(|_| {});
        let late_configs = Some(Default::default());

        Self {
            plugins,
            runner,
            setup_handler,
            event_handler,
            late_configs,
        }
    }

    pub fn add_config<C>(mut self, config: C) -> Self
    where
        C: BuildConfig<S> + 'static,
    {
        if config.late_evaluation() {
            if let Some(late_configs) = &mut self.late_configs {
                let typ = std::any::TypeId::of::<C>();
                late_configs.insert(typ, Box::new(config));
            }

            return self;
        }

        config.apply(self)
    }

    pub fn set_event<T, H>(mut self, mut handler: H) -> Self
    where
        H: Handler<S, T> + 'static,
    {
        self.event_handler = Box::new(move |storage| handler.call(storage));
        self
    }

    pub fn set_runner<F: FnMut(App<S>) -> Result<(), String> + 'static>(
        mut self,
        runner: F,
    ) -> Self {
        self.runner = Box::new(runner);
        self
    }

    pub fn add_plugin<T: 'static>(mut self, plugin: T) -> Self {
        self.plugins.add(plugin);
        self
    }

    pub fn build(mut self) -> Result<(), String> {
        if let Some(late_configs) = self.late_configs.take() {
            for (_, config) in late_configs {
                self = config.apply(self);
            }
        }

        let Self {
            mut plugins,
            mut runner,
            setup_handler,
            event_handler,
            ..
        } = self;

        let state = (setup_handler)(&mut plugins)?;
        let storage = Storage { plugins, state };

        let app = App {
            storage,
            events: vec![],
            event_handler,
        };

        (runner)(app)?;

        Ok(())
    }
}

fn default_runner<S: GKState>(_app: App<S>) -> Result<(), String> {
    // TODO: logic here?
    Ok(())
}

//---
use crate::config::BuildConfig;
use anymap::AnyMap;
use indexmap::IndexMap;

pub struct Storage<S: GKState> {
    pub state: S,
    pub plugins: Plugins,
}

pub struct Plugins {
    map: AnyMap,
}

impl Plugins {
    fn new() -> Self {
        Self { map: AnyMap::new() }
    }

    fn add<T: 'static>(&mut self, plugin: T) {
        self.map.insert(plugin);
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
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

pub trait Plugin {}
pub trait GKState {}

pub trait FromStorage<S: GKState> {
    fn from_storage<'gk_state>(app: &'gk_state mut Storage<S>) -> &'gk_state mut Self;
}

impl<S: GKState, T: Plugin + 'static> FromStorage<S> for T {
    fn from_storage(storage: &mut Storage<S>) -> &mut Self {
        storage.plugins.map.get_mut::<Self>().unwrap()
    }
}

pub trait Handler<S: GKState, T> {
    fn call(&mut self, app: &mut Storage<S>);
}

pub trait SetupHandler<S, T> {
    fn call(self, storage: &mut Plugins) -> Result<S, String>;
}

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_handler ({ $($param:ident)* } => {
    impl<S, Fun, $($param,)*> Handler<S, ($($param,)*)> for Fun
    where
        S: GKState + 'static,
        Fun: FnMut($(&mut $param),*),
        $($param:FromStorage<S> + 'static),*
    {
        fn call(&mut self, storage: &mut Storage<S>) {
            // Look for duplicated parameters and panic
            #[cfg(debug_assertions)]
            {
                use std::collections::HashSet;
                use std::any::TypeId;
                let mut h_set:HashSet<TypeId> = Default::default();

                $(

                    if !h_set.insert(TypeId::of::<$param>()) {
                        panic!("Application handlers cannot contains duplicated parameters.");
                    }
                )*
            }


            // Safety. //TODO
            paste::paste! {
                let ($([<$param:lower _v>],)*) = unsafe {
                    $(let [<$param:lower _v>] = $param::from_storage(storage) as *mut _;)*
                    ($(&mut *[<$param:lower _v>],)*)
                };
                (self)($([<$param:lower _v>],)*);
            }
        }
    }
});

fn_handler! {}
fn_handler! { A }
fn_handler! { A B }
fn_handler! { A B C }
fn_handler! { A B C D }
fn_handler! { A B C D E }
fn_handler! { A B C D E F }
fn_handler! { A B C D E F G }
fn_handler! { A B C D E F G H }
fn_handler! { A B C D E F G H I }
fn_handler! { A B C D E F G H I J }

//-

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_setup_handler ({ $($param:ident)* } => {
    impl<S, Fun, $($param,)*> SetupHandler<S, ($($param,)*)> for Fun
    where
        S: 'static,
        Fun: FnOnce($(&mut $param),*) -> Result<S, String>,
        $($param:FromPlugins + 'static),*
    {
        fn call(mut self, plugins: &mut Plugins) -> Result<S, String> {
            // Look for duplicated parameters and panic
            #[cfg(debug_assertions)]
            {
                use std::collections::HashSet;
                use std::any::TypeId;
                let mut h_set:HashSet<TypeId> = Default::default();

                $(

                    if !h_set.insert(TypeId::of::<$param>()) {
                        panic!("Application handlers cannot contains duplicated parameters.");
                    }
                )*
            }


            // Safety. //TODO
            paste::paste! {
                let ($([<$param:lower _v>],)*) = unsafe {
                    $(let [<$param:lower _v>] = $param::from_plugins(plugins) as *mut _;)*
                    ($(&mut *[<$param:lower _v>],)*)
                };
                return (self)($([<$param:lower _v>],)*);
            }
        }
    }
});

fn_setup_handler! {}
fn_setup_handler! { A }
fn_setup_handler! { A B }
fn_setup_handler! { A B C }
fn_setup_handler! { A B C D }
fn_setup_handler! { A B C D E }
fn_setup_handler! { A B C D E F }
fn_setup_handler! { A B C D E F G }
fn_setup_handler! { A B C D E F G H }
fn_setup_handler! { A B C D E F G H I }
fn_setup_handler! { A B C D E F G H I J }
