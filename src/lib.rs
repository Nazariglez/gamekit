pub mod m2d;
pub mod prelude;
pub mod tween;

pub use gk_app as app;
use gk_app::GKState;
pub use gk_gfx as gfx;
pub use gk_platform as platform;

pub fn init() -> app::AppBuilder<()> {
    app::AppBuilder::init()
}

pub fn init_with<S, T, H>(handler: H) -> app::AppBuilder<S>
where
    S: GKState + 'static,
    H: app::handlers::SetupHandler<S, T> + 'static,
{
    app::AppBuilder::init_with(handler)
}
