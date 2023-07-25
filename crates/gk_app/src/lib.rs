mod app;
mod builder;
mod config;
mod runner;
mod utils;

pub mod event;
pub mod handlers;
pub mod prelude;
pub mod storage;
pub mod window;

pub use app::App;
pub use builder::AppBuilder;
pub use config::BuildConfig;
pub use event::EventQueue;

pub use gk_macro::AppState;

/// Represents an App's plugin
pub trait Plugin {}

/// Represents an App's state
pub trait GKState {}
