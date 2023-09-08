use super::waker::*;
use crate::events::{AssetLoad, AssetState};
use futures::future::LocalBoxFuture;
use futures::task::{Context, Poll};
use futures::TryFutureExt;
use gk_app::{event, AppBuilder, BuildConfig, EventQueue, GKState, Plugin};

// TODO url loader
// TODO move platter2 logic here...

pub struct AssetLoader {
    loading: Vec<LoadWrapper>,
}

impl AssetLoader {
    pub fn config() -> AssetLoaderConfig {
        AssetLoaderConfig::default()
    }

    pub(crate) fn update<S: GKState + 'static>(&mut self, events: &mut EventQueue<S>) {
        let mut needs_clean = true;

        self.loading.iter_mut().for_each(|loader| {
            if let Some(loaded) = loader.try_load() {
                events.queue(loaded);
                needs_clean = true;
            }
        });

        if needs_clean {
            self.loading.retain(|loader| !loader.is_loaded());
        }
    }

    pub fn load(&mut self, file_path: &str) -> &mut Self {
        log::info!("Loading file '{}'", file_path);
        let id = file_path.to_string(); // todo avoid to_string allocations
        let fut = Box::pin(platter2::load_file(file_path.to_string()).map_err(|e| e.to_string()));
        self.loading.push(LoadWrapper::new(file_path, fut));
        self
    }
}

impl Default for AssetLoader {
    fn default() -> AssetLoader {
        AssetLoader { loading: vec![] }
    }
}

impl Plugin for AssetLoader {}

#[derive(Debug, Default, Copy, Clone)]
pub struct AssetLoaderConfig;

impl<S: GKState + 'static> BuildConfig<S> for AssetLoaderConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let builder = builder.on(
            |_: &event::FrameStart, loader: &mut AssetLoader, events: &mut EventQueue<S>| {
                loader.update(events)
            },
        );
        Ok(builder.add_plugin(AssetLoader::default()))
    }
}

struct LoadWrapper {
    id: String,
    fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>,
    loaded: bool,
}

impl LoadWrapper {
    pub fn new(id: &str, fut: LocalBoxFuture<'static, Result<Vec<u8>, String>>) -> Self {
        Self {
            id: id.to_string(),
            fut,
            loaded: false,
        }
    }

    pub fn try_load(&mut self) -> Option<AssetLoad> {
        let waker = DummyWaker.into_task_waker();
        let mut ctx = Context::from_waker(&waker);
        match self.fut.as_mut().poll(&mut ctx) {
            Poll::Ready(r_buff) => {
                self.loaded = true;
                match r_buff {
                    Ok(buff) => Some(AssetLoad {
                        id: self.id.clone(),
                        state: AssetState::Loaded(buff),
                    }),
                    Err(err) => {
                        let err = format!("Cannot load file: {}: {}", self.id, err);
                        log::warn!("{}", err);
                        Some(AssetLoad {
                            id: self.id.clone(),
                            state: AssetState::Err(err),
                        })
                    }
                }
            }
            _ => None,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
}
