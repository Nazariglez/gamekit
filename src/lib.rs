// pub mod m2d;
pub mod prelude;
pub mod time;
pub mod utils;
// pub mod tween;

pub use gk_app as app;
pub use gk_backend as platform;
pub use gk_gfx as gfx;

use gk_app::GKState;

pub fn init() -> app::AppBuilder<()> {
    simple_logger::SimpleLogger::new()
        .without_timestamps()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    app::AppBuilder::init()
}

pub fn init_with<S, T, H>(handler: H) -> app::AppBuilder<S>
where
    S: GKState + 'static,
    H: app::handlers::SetupHandler<S, T> + 'static,
{
    simple_logger::SimpleLogger::new()
        .without_timestamps()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    app::AppBuilder::init_with(handler)
}
