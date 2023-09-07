use crate::utils::LoadWrapper;
use crate::AssetLoaded;
use futures::{TryFuture, TryFutureExt};
use gk_app::{event, AppBuilder, BuildConfig, GKState, Plugin};
use hashbrown::HashMap;

pub struct Assets {
    loading: Vec<LoadWrapper>,
    loaded: Vec<AssetLoaded>,
}

impl Assets {
    pub fn update(&mut self) { // todo pub(crate)?
        let mut needs_clean = true;

        self.loading.iter_mut().for_each(|loader| {
            if let Some(loaded) = loader.try_load() {
                self.loaded.push(loaded);
                needs_clean = true;
                log::info!("Loaded!");
            }
        });

        if needs_clean {
            self.loading.retain(|loader| !loader.is_loaded());
        }
    }

    pub fn load(&mut self, file_path: &str) {
        log::info!("Loading file '{}'", file_path);
        let id = file_path.to_string(); // todo avoid to_string allocations
        let fut = Box::pin(platter2::load_file(file_path.to_string()).map_err(|e| e.to_string()));
        self.loading.push(LoadWrapper::new(file_path, fut));
    }
}

impl Default for Assets {
    fn default() -> Assets {
        Assets {
            loading: vec![],
            loaded: vec![]
        }
    }
}

impl Assets {
    pub fn config() -> AssetsConfig {
        AssetsConfig::default()
    }
}

impl Plugin for Assets {}

#[derive(Debug, Default, Copy, Clone)]
pub struct AssetsConfig;

impl<S: GKState + 'static> BuildConfig<S> for AssetsConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        // let builder = builder.on(|_: &event::FrameStart, time: &mut Time| time.update());
        Ok(builder.add_plugin(Assets::default()))
    }
}
