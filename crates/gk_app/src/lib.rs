#[cfg(not(feature = "winit"))]
mod empty;

#[cfg(not(feature = "winit"))]
pub use crate::empty::*;

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "winit")]
pub use crate::winit::*;

mod app;
mod config;

pub use app::*;
pub use config::*;
