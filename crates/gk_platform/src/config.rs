use crate::empty::*;
use crate::GKWindowAttributes;
use gk_app::event::AppEvent;
use gk_app::{AppBuilder, BuildConfig, GKState};

pub struct PlatformConfig;

impl<S: GKState> BuildConfig<S> for PlatformConfig {
    fn apply(&mut self, mut builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        // builder = match self.main_window.take() {
        //     None => builder,
        //     Some(attrs) => {
        //         builder.listen_event(|evt: &AppEvent| {
        //             match  { }
        //         })
        //     }
        // }
        let windows = Windows::default();
        Ok(builder.add_plugin(windows).with_runner(runner))
    }
}
