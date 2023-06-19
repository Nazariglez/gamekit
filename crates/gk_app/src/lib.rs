use std::marker::PhantomData;
use gk_core::{GKWindow, GKWindowId, GKWindowManager};
use std::ops::Rem;

pub struct App<S> {
    storage: Storage,
    events: Vec<()>,
    _s: PhantomData<S>,
    // state: S,
}

impl<S> App<S> {
    pub fn get_mut_plugin<T: 'static>(&mut self) -> Option<&mut T> {
        self.storage.get_mut()
    }

    pub fn tick(&mut self) {
        println!("TICK!!");
    }
}

pub struct AppBuilder<S: 'static> {
    storage: Storage,
    runner: Box<dyn FnMut(App<S>) -> Result<(), String>>,
    setup_handler: Box<dyn FnOnce(&mut Storage)>,
}

impl AppBuilder<()> {
    pub fn init() -> Self {
        Self::init_with(|| {})
    }
}

impl<S> AppBuilder<S> {
    pub fn init_with<T, H>(handler: H) -> Self
    where
    H: Handler<T> + 'static,
    {
        let mut storage = Storage::new();
        let runner = Box::new(default_runner);
        let setup_handler: Box<dyn FnOnce(&mut Storage)> =
            Box::new(|storage| handler.call(storage));

        Self {
            storage,
            runner,
            setup_handler,
        }
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

        // let state = (setup_handler)(&mut storage)?;

        let app = App {
            storage,
            events: vec![],
            _s: PhantomData::default(),
            // state,
        };

        (runner)(app)?;

        Ok(())
    }
}

fn default_runner<S>(_app: App<S>) -> Result<(), String> {
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
        println!(
            "-> {:?}, -> {:?}",
            std::any::TypeId::of::<Self>(),
            storage.map
        );
        storage.map.get_mut::<Self>().unwrap()
    }
}

pub trait Handler<T> {
    fn call(self, storage: &mut Storage);
}

pub trait SetupHandler<S, T> {
    fn call(self, storage: &mut Storage) -> Result<S, String>;
}

pub fn dispatch<T, H>(storage: &mut Storage, handler: H)
where
    H: Handler<T>,
{
    handler.call(storage);
}

// Safe for notan because the map will never change
// once it's created it will not have new register or removed ones
// Doing this we got interior mutability for the components but not the map
// because is never exposes
macro_rules! fn_handler ({ $($param:ident)* } => {
    impl<Fun, $($param,)*> Handler<($($param,)*)> for Fun
    where
        Fun: FnMut($(&mut $param),*),
        $($param:FromStorage + 'static),*
    {
        fn call(mut self, storage: &mut Storage) {
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

fn_handler! {  }
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