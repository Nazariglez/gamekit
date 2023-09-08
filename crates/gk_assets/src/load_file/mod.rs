#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
use web::raw_load_file;

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
use native::raw_load_file;

use std::{future::Future, io::Error as IOError};

pub(crate) fn load_file(path: &str) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    raw_load_file(path)
}