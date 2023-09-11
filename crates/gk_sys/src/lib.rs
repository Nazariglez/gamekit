mod builder;
mod config;
mod runner;
mod sys;
mod utils;

pub mod event;
pub mod handlers;
pub mod keyboard;
pub mod mouse;
pub mod prelude;
pub mod storage;
pub mod window;

pub use builder::AppBuilder;
pub use config::BuildConfig;
pub use event::EventQueue;
pub use sys::System;

pub use gk_macro::AppState;

/// Represents an App's plugin
pub trait Plugin {}

/// Represents an App's state
pub trait GKState {}
