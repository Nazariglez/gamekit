#[cfg(not(feature = "wgpu"))]
mod empty;

#[cfg(not(feature = "wgpu"))]
pub use crate::empty::*;

#[cfg(feature = "wgpu")]
mod wgpu;

#[cfg(feature = "wgpu")]
pub use crate::wgpu::*;

mod gfx;
mod device;
mod renderer;
mod color;
mod config;

pub use device::*;
pub use color::Color;
pub use config::*;
pub use renderer::*;
pub use gfx::*;

