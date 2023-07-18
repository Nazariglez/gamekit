use crate::window::{GKWindowAttributes, WindowEvent, WindowEventId};
use crate::{backend, Platform};
use gk_app::event::AppEvent;
use gk_app::{AppBuilder, BuildConfig, GKState};

pub struct PlatformConfig {
    main_window: Option<GKWindowAttributes>,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self::with_window(Default::default())
    }
}

impl PlatformConfig {
    pub fn windowless() -> Self {
        Self { main_window: None }
    }

    pub fn with_window(attrs: GKWindowAttributes) -> Self {
        Self {
            main_window: Some(attrs),
        }
    }
}

impl<S: GKState> BuildConfig<S> for PlatformConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        // start the app with a window
        let builder = match self.main_window.take() {
            None => builder,
            Some(attrs) => builder.listen_event_once(
                move |evt: &AppEvent, platform: &mut Platform| match evt {
                    AppEvent::Init => {
                        let id = platform.create_window(attrs).unwrap();
                        platform.set_main_window(id);
                    }
                    _ => {}
                },
            ),
        };

        // Read windows event to set main window and close app when all windows are closed
        let builder = builder.listen_event(|evt: &WindowEvent, platform: &mut Platform| match evt
            .event
        {
            // WindowEventId::Open => windows.set_main_window(evt.id),
            WindowEventId::FocusGained => platform.set_main_window(evt.id),
            WindowEventId::Close => {
                if platform.window_ids().len() == 0 {
                    platform.exit();
                }
            }
            _ => {}
        });

        // let's add the windows plugin
        let platform = Platform::new();
        Ok(builder.add_plugin(platform).with_runner(backend::runner))
    }
}
