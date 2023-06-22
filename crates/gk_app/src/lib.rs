use gk_core::{GKWindow, GKWindowId, GKWindowManager};
use std::marker::PhantomData;
use std::ops::Rem;

pub struct App<S: GKState> {
    storage: Storage<S>,
    events: Vec<()>,
    event_handler: Box<dyn FnMut(&mut Storage<S>)>,
}

impl<S: GKState> App<S> {
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.plugins.get_mut()
    }

    pub fn tick(&mut self) {
        println!("TICK!!");
        (self.event_handler)(&mut self.storage);
    }
}

pub struct AppBuilder<S: GKState + 'static> {
    plugins: Plugins,
    runner: Box<dyn FnMut(App<S>) -> Result<(), String>>,
    setup_handler: Box<dyn FnOnce(&mut Plugins) -> Result<S, String>>,
    event_handler: Box<dyn FnMut(&mut Storage<S>)>,
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
        let setup_handler: Box<dyn FnOnce(&mut Plugins) -> Result<S, String>> =
            Box::new(|plugins| handler.call(plugins));
        let event_handler: Box<dyn FnMut(&mut Storage<S>)> = Box::new(|_| {});

        Self {
            plugins,
            runner,
            setup_handler,
            event_handler,
        }
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

    pub fn run(mut self) -> Result<(), String> {
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
use anymap::AnyMap;

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
        println!("add {:?} -> {:?}", std::any::TypeId::of::<T>(), self.map);
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        println!("get {:?} -> {:?}", std::any::TypeId::of::<T>(), self.map);
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

                println!("{:?}", TypeId::of::<$param>());
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

                println!("{:?}", TypeId::of::<$param>());
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
