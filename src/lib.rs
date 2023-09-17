// pub mod m2d;
pub mod prelude;

#[cfg(feature = "random")]
pub mod random;
pub mod spritebatch;
pub mod time;
pub mod utils;
// pub mod tween;

pub use gk_app as app;
pub use gk_assets as assets;
pub use gk_gfx as gfx;
pub use gk_math as math;
pub use gk_sys as sys;

use gk_sys::GKState;

pub fn init() -> sys::AppBuilder<()> {
    // simple_logger::SimpleLogger::new()
    //     .without_timestamps()
    //     .with_level(log::LevelFilter::Debug)
    //     .init()
    //     .unwrap();
    sys::AppBuilder::init()
}

pub fn init_with<S, T, H>(handler: H) -> sys::AppBuilder<S>
where
    S: GKState + 'static,
    H: sys::handlers::SetupHandler<S, T> + 'static,
{
    // simple_logger::SimpleLogger::new()
    //     .without_timestamps()
    //     .with_level(log::LevelFilter::Debug)
    //     .init()
    //     .unwrap();
    sys::AppBuilder::init_with(handler)
}
