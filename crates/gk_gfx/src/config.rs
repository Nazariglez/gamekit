use crate::gfx::Gfx;
use crate::GfxAttributes;
use gk_app::window::{WindowEvent, WindowEventId};
use gk_app::{AppBuilder, BuildConfig, GKState};
use gk_backend::Platform;

#[derive(Default)]
pub struct GfxConfig {
    attrs: GfxAttributes,
}

impl GfxConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Use VSync mode if possible
    pub fn with_vsync(mut self, enable: bool) -> Self {
        self.attrs.vsync = enable;
        self
    }
}

impl<S: GKState + 'static> BuildConfig<S> for GfxConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let builder = builder.on(
            |evt: &WindowEvent, gfx: &mut Gfx, platform: &mut Platform| match evt.event {
                // when a new window is created let's initialize the surface for it
                WindowEventId::Init => {
                    gfx.init_surface(platform.window(evt.id).unwrap()).unwrap();
                }
                WindowEventId::Moved { .. } => {}
                WindowEventId::Resized {
                    width,
                    height,
                    scale_factor,
                } => {
                    let w = (width as f64 * scale_factor) as u32;
                    let h = (height as f64 * scale_factor) as u32;
                    gfx.resize(evt.id, w, h);
                }
                WindowEventId::Minimized => {}
                WindowEventId::Maximized => {}
                WindowEventId::FocusGained => {}
                WindowEventId::FocusLost => {}
                WindowEventId::Close => {}
            },
        );

        let attrs = self.attrs;
        Ok(builder.add_plugin_with(move |platform: &mut Platform| {
            let mut gfx = Gfx::new(attrs)?;
            if let Some(win) = platform.main_window() {
                let _ = gfx.init_surface(win)?;
            }

            Ok(gfx)
        })?)
    }
}
