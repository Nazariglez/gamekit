use crate::empty::*;
use gk_app::{AppBuilder, BuildConfig, GKState};

pub struct PlatformConfig;

impl<S: GKState> BuildConfig<S> for PlatformConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let windows = Windows::default();
        Ok(builder.add_plugin(windows).with_runner(runner))
    }
}
