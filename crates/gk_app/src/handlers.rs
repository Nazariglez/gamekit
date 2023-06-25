use crate::storage::{FromPlugins, FromStorage, Plugins, Storage};
use crate::{App, GKState};

pub type RunnerHandlerFn<S> = dyn FnMut(App<S>) -> Result<(), String>;
pub type EventHandlerFn<S> = dyn FnMut(&mut Storage<S>);
pub type SetupHandlerFn<S> = dyn FnOnce(&mut Plugins) -> Result<S, String>;

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
