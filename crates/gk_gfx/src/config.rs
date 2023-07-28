use crate::gfx::Gfx;
use gk_app::window::{WindowEvent, WindowEventId};
use gk_app::{AppBuilder, BuildConfig, GKState};
use gk_backend::Platform;

pub struct GfxConfig;

impl Default for GfxConfig {
    fn default() -> Self {
        GfxConfig
    }
}

impl<S: GKState + 'static> BuildConfig<S> for GfxConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let builder = builder.on(
            |evt: &WindowEvent, gfx: &mut Gfx, platform: &mut Platform| match evt.event {
                WindowEventId::Init => {
                    gfx.init_context(platform.window(evt.id).unwrap()).unwrap();
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

        let gfx = Gfx::new()?;
        Ok(builder.add_plugin(gfx))
    }
}
