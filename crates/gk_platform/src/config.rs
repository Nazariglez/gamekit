use crate::{GKWindowAttributes, WindowEvent, WindowEventId};
use gk_app::event::AppEvent;
use gk_app::{AppBuilder, BuildConfig, GKState};

#[cfg(feature = "empty")]
use crate::empty::*;

#[cfg(feature = "winit")]
use crate::winit::*;

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

        // Read windows event to set main window and close app when all windows are closed
        let builder =
            builder.listen_event(|evt: &WindowEvent, windows: &mut Windows| match evt.event {
                // WindowEventId::Open => windows.set_main_window(evt.id),
                WindowEventId::FocusGained => windows.set_main_window(evt.id),
                WindowEventId::Close => {
                    if windows.window_ids().len() == 0 {
                        windows.exit();
                    }
                }
                _ => {}
            });

        // let's add the windows plugin
        let windows = Windows::new();
        Ok(builder.add_plugin(windows).with_runner(runner))
    }
}
