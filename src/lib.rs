// pub mod m2d;
pub mod prelude;
// pub mod tween;

pub use gk_app as app;
pub use gk_backend as platform;
pub use gk_gfx as gfx;

use gk_app::GKState;

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
