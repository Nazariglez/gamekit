use crate::empty::*;
use crate::GKWindowAttributes;
use gk_app::event::AppEvent;
use gk_app::{AppBuilder, BuildConfig, GKState};

#[derive(Default)]
pub struct PlatformConfig {
    main_window: Option<GKWindowAttributes>,
}

impl PlatformConfig {
    pub fn window(mut self, attrs: GKWindowAttributes) -> Self {
        self.main_window = Some(attrs);
        self
    }
}

impl<S: GKState> BuildConfig<S> for PlatformConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        // start the app with a window
        let builder = match self.main_window.take() {
            None => builder,
            Some(attrs) => {
                builder.listen_event_once(move |evt: &AppEvent, windows: &mut Windows| match evt {
                    AppEvent::Init => {
                        let mut builder = windows.create();
                        builder.attrs = attrs;
                        let id = builder.build().unwrap();
                        windows.set_main_window(id);
                    }
                    _ => {}
                })
            }
        };

        // let's add the windows plugin
        let windows = Windows::default();
        Ok(builder.add_plugin(windows).with_runner(runner))
    }
}
