#![cfg(feature = "winit")]

mod event_loop;
mod manager;
mod runner;
mod utils;
mod window;

use crate::platform::WindowManager;

pub use manager::*;
pub use runner::*;
pub use window::*;

pub type Windows = WindowManager<Window, Manager>;
