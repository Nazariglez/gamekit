use gk_core::{GKWindow, GKWindowId, GKWindowManager};
use std::marker::PhantomData;
use std::ops::Rem;

pub struct App<S: GKState> {
    storage: Storage,
    events: Vec<()>,
    pub state: S,
    event_handler: Box<dyn FnMut(&mut App<S>)>,
}

impl<S: GKState> App<S> {
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.get_mut()
    }

    pub fn tick(&mut self) {
        println!("TICK!!");
        // (self.event_handler)(self);
    }
}

pub struct AppBuilder<S: GKState + 'static> {
    storage: Storage,
    runner: Box<dyn FnMut(App<S>) -> Result<(), String>>,
    setup_handler: Box<dyn FnOnce(&mut Storage) -> Result<S, String>>,
    event_handler: Box<dyn FnMut(&mut App<S>)>,
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
        let mut storage = Storage::new();
        let runner = Box::new(default_runner);
        let setup_handler: Box<dyn FnOnce(&mut Storage) -> Result<S, String>> =
            Box::new(|storage| handler.call(storage));
        let event_handler: Box<dyn FnMut(&mut App<S>)> = Box::new(|app| {});

        Self {
            storage,
            runner,
            setup_handler,
            event_handler,
        }
    }

    pub fn set_event<T, H>(mut self, mut handler: H) -> Self
    where
        H: Handler<S, T> + 'static,
    {
        self.event_handler = Box::new(move |app| handler.call(app));
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
        self.storage.add(plugin);
        self
    }

    pub fn run(mut self) -> Result<(), String> {
        let Self {
            mut storage,
            mut runner,
            setup_handler,
            event_handler,
            ..
        } = self;
        // dispatch(&mut storage, |manager: &mut WM| {
        //     println!("yep!");
        //     // manager.create().unwrap();
        // });
        //
        // let mut count = 0;
        // let runner = manager.create_runner(move |mut manager| {
        //     if count < 3000 && count.rem(1000) == 0 {
        //         manager.create();
        //     }
        //
        //     count += 1;
        // });
        // self.manager.run(|| {});

        let state = (setup_handler)(&mut storage)?;

        let app = App {
            storage,
            events: vec![],
            state,
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

pub struct Storage {
    map: AnyMap,
}

impl Storage {
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

pub trait FromStorage {
    fn from_storage(storage: &mut Storage) -> &mut Self;
}

impl<T: 'static> FromStorage for T {
    fn from_storage(storage: &mut Storage) -> &mut Self {
        storage.map.get_mut::<Self>().unwrap()
    }
}

pub trait Plugin {}
pub trait GKState {}

pub trait FromAppStorage<S: GKState> {
    fn from_storage(app: &mut App<S>) -> &mut Self;
}

impl<S: GKState, T: Plugin + 'static> FromAppStorage<S> for T {
    fn from_storage(app: &mut App<S>) -> &mut Self {
        app.storage.map.get_mut::<Self>().unwrap()
    }
}

pub trait Handler<S: GKState, T> {
    fn call(&mut self, app: &mut App<S>);
}

pub trait SetupHandler<S, T> {
    fn call(self, storage: &mut Storage) -> Result<S, String>;
}

// pub fn dispatch<S, T, H>(storage: &mut Storage, handler: H)
// where
//     H: Handler<S, T>,
// {
//     handler.call(storage);
// }

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_handler ({ $($param:ident)* } => {
    impl<S, Fun, $($param,)*> Handler<S, ($($param,)*)> for Fun
    where
        S: GKState + 'static,
        Fun: FnMut($(&mut $param),*),
        $($param:FromAppStorage<S> + 'static),*
    {
        fn call(&mut self, app: &mut App<S>) {
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
                    $(let [<$param:lower _v>] = $param::from_storage(app) as *mut _;)*
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
        Fun: FnMut($(&mut $param),*) -> Result<S, String>,
        $($param:FromStorage + 'static),*
    {
        fn call(mut self, storage: &mut Storage) -> Result<S, String> {
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
