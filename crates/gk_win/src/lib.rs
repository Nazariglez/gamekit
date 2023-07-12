mod config;
mod windows;

pub use crate::config::WindowsConfig;
pub use crate::windows::WindowManager;

#[cfg(feature = "gk_winit")]
#[cfg(feature = "gk_winit")]
pub(crate) use gk_winit::runner as app_runner;
