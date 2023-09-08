#![cfg(not(target_arch = "wasm32"))]

pub fn raw_load_file(path: &str)  -> impl Future<Output = Result<Vec<u8>, IOError>> {
    futures_util::future::ready(read(path))
}