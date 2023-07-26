use crate::{runner, Platform};
use gk_app::event;
use gk_app::window::{GKWindow, GKWindowAttributes, WindowEvent, WindowEventId};
use gk_app::{AppBuilder, BuildConfig, GKState};

pub struct PlatformConfig {
    main_window: Option<GKWindowAttributes>,
    auto_redraw: bool,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self::with_window(Default::default())
    }
}

impl PlatformConfig {
    pub fn windowless() -> Self {
        Self {
            main_window: None,
            auto_redraw: true,
        }
    }

    pub fn with_window(attrs: GKWindowAttributes) -> Self {
        Self {
            main_window: Some(attrs),
            auto_redraw: true,
        }
    }
}

impl<S: GKState> BuildConfig<S> for PlatformConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        // start the app with a window
        let builder = match self.main_window.take() {
            None => builder,
            Some(attrs) => builder.once(move |evt: &event::Init, platform: &mut Platform| {
                let id = platform.create_window(attrs).unwrap();
                platform.set_main_window(id);
            }),
        };

        let builder = if self.auto_redraw {
            builder.on(|_: &event::Update, platform: &mut Platform| {
                platform.windows_mut().for_each(|win| win.request_redraw())
            })
        } else {
            builder
        };

        // Read windows event to set main window and close app when all windows are closed
        let builder = builder.on(
            |evt: &WindowEvent, platform: &mut Platform| match evt.event {
                // WindowEventId::Open => windows.set_main_window(evt.id),
                WindowEventId::FocusGained => platform.set_main_window(evt.id),
                WindowEventId::Close => {
                    if platform.window_ids().len() == 0 {
                        platform.exit();
                    }
                }
                _ => {}
            },
        );

        // let's add the windows plugin
        let platform = Platform::new();
        Ok(builder.add_plugin(platform).with_runner(runner))
    }
}
