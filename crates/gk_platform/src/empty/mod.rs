#![cfg(feature = "empty")]

mod runner;
mod window;

use crate::platform::WindowManager;

pub use runner::*;
pub use window::*;

pub type Windows = WindowManager<Window, Manager>;
