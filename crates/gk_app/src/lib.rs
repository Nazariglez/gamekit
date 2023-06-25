mod app;
mod builder;
mod config;
mod runner;

pub mod handlers;
pub mod prelude;
pub mod storage;

pub use app::App;
pub use builder::AppBuilder;
pub use config::BuildConfig;

pub use gk_macro::AppState;

/// Represents an App's plugin
pub trait Plugin {}

/// Represents an App's state
pub trait GKState {}
