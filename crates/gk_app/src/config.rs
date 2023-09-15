use crate::{runner, App};
use gk_sys::event;
use gk_sys::window::{GKWindow, WindowAction, WindowAttributes, WindowEvent};
use gk_sys::{AppBuilder, BuildConfig, GKState};

pub struct PlatformConfig {
    main_window: Option<WindowAttributes>,
    auto_redraw: bool,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            main_window: Some(Default::default()),
            auto_redraw: true,
        }
    }
}

impl PlatformConfig {
    pub fn with_windowless(mut self) -> Self {
        self.main_window = None;
        self
    }

    pub fn with_window(mut self, attrs: WindowAttributes) -> Self {
        self.main_window = Some(attrs);
        self
    }
}

impl<S: GKState> BuildConfig<S> for PlatformConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let mut platform = App::new();

        // Initialize main windows if is not windowless mode
        if let Some(attrs) = self.main_window.take() {
            let id = platform.create_window(attrs)?;
            log::info!("Window '{:?}' created.", id);
        }

        // Call request_draw on each frame
        let builder = if self.auto_redraw {
            builder.on(|_: &event::UpdateEvent, platform: &mut App| {
                platform.windows_mut().for_each(|win| win.request_redraw())
            })
        } else {
            builder
        };

        // Read windows event to set main window and close app when all windows are closed
        let builder = builder.on(|evt: &WindowEvent, platform: &mut App| match evt.action {
            WindowAction::Close => {
                if platform.window_ids().is_empty() {
                    platform.exit();
                }
            }
            _ => {}
        });

        // let's add the windows plugin
        Ok(builder.add_plugin(platform).with_runner(runner))
    }
}
