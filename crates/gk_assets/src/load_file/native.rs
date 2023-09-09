use futures::channel::oneshot;
use std::future::Future;

pub(crate) struct FileLoader {
    #[cfg(not(target_arch = "wasm32"))]
    thread_pool: rayon::ThreadPool,
}

impl FileLoader {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Result<Self, String> {
        let thread_pool = rayon::ThreadPoolBuilder::default()
            .num_threads(10)
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Self { thread_pool })
    }

    pub fn load_file(&self, path: &str) -> impl Future<Output = Result<Vec<u8>, String>> {
        let (tx, rx) = oneshot::channel();

        let path = path.to_owned();

        // Spawn the thread.
        self.thread_pool.spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let read_result = std::fs::read(&path);
            let _ = tx.send(read_result.map_err(|e| e.to_string()));
        });

        // Convert the receiver into a Future.
        async move {
            match rx.await {
                Ok(result) => result,
                Err(_) => Err("The channel was dropped.".to_string()),
            }
        }
    }
}
