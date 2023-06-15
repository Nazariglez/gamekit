use gk_core::{GKWindow, GKWindowId, GKWindowManager};
use gk_winit::{EventLoopWindowTarget, Manager, Window};
use std::marker::PhantomData;
use std::ops::Rem;

pub struct Builder<W: GKWindow, WM: GKWindowManager<W>> {
    storage: Storage,
    _w: PhantomData<W>,
    manager: WM,
}

impl<W: GKWindow, WM: GKWindowManager<W> + 'static> Builder<W, WM> {
    pub fn new() -> Result<Self, String> {
        let mut manager = WM::new()?;
        let mut storage = Storage::new();
        let manager2 = WM::new()?;
        storage.add(manager2);

        Ok(Self {
            storage,
            manager,
            _w: PhantomData::default(),
        })
    }

    pub fn run(mut self) {
        let Self {
            manager,
            mut storage,
            ..
        } = self;
        dispatch(&mut storage, |manager: &mut WM| {
            println!("yep!");
            // manager.create().unwrap();
        });

        let mut count = 0;
        let runner = manager.create_runner(move |mut manager| {
            if count < 3000 && count.rem(1000) == 0 {
                manager.create();
            }

            count += 1;
        });
        // self.manager.run(|| {});
        runner();
    }
}

pub fn init() -> Result<Builder<Window, Manager>, String> {
    Builder::new()
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
        println!("{:?}", self.map);
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
                let mut h_set = HashSet::new();

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
