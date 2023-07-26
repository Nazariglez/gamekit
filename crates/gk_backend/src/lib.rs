#[cfg(not(feature = "winit"))]
mod empty;

#[cfg(not(feature = "winit"))]
pub use crate::empty::*;

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "winit")]
pub use crate::winit::*;

mod config;
mod platform;

pub use config::*;
pub use platform::*;
