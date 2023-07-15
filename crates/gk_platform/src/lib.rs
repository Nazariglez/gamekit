mod platform;
pub mod prelude;
mod window;

#[cfg(feature = "empty")]
mod empty;

#[cfg(feature = "winit")]
mod winit;

#[cfg(any(feature = "empty", feature = "winit"))]
mod config;

pub use platform::*;
pub use window::*;

#[cfg(feature = "empty")]
pub use empty::*;

#[cfg(feature = "winit")]
pub use crate::winit::*;

#[cfg(any(feature = "empty", feature = "winit"))]
pub use config::*;
