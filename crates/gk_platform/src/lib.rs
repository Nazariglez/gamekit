pub mod prelude;

mod platform;
mod window;

#[cfg(not(feature = "winit"))]
mod empty;

#[cfg(not(feature = "winit"))]
pub(crate) use crate::empty as backend;

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "winit")]
pub(crate) use crate::winit as backend;

mod config;

pub use platform::*;
pub use window::*;

pub use config::*;
