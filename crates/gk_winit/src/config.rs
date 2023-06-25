use crate::{runner, Manager};
use gk_app::{AppBuilder, BuildConfig, GKState};

pub struct WinitConfig;

impl<S: GKState> BuildConfig<S> for WinitConfig {
    fn apply(&self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        Ok(builder.add_plugin(Manager::new()).set_runner(runner))
    }
}
