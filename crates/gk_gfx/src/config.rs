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

    /// Use of the integrated gpu if possible
    pub fn with_integrated_gpu(mut self, value: bool) -> Self {
        self.attrs.integrated_gpu = value;
        self
    }

    /// GPU limits compatible with webgl2, D3D11 and GLES-3.0
    pub fn with_compatible_mode(mut self, compatible: bool) -> Self {
        self.attrs.compatible_mode = compatible;
        self
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

        let attrs = self.attrs;
        Ok(builder.add_plugin_with(move |platform: &mut Platform| {
            let mut gfx = Gfx::new(attrs)?;
            if let Some(win) = platform.main_window() {
                let _ = gfx.init_context(win)?;
            }

            Ok(gfx)
        })?)
    }
}