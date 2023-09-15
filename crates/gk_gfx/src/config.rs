use crate::gfx::Gfx;
use crate::GfxAttributes;
use gk_app::App;
use gk_sys::window::{WindowAction, WindowEvent};
use gk_sys::{AppBuilder, BuildConfig, EventQueue, GKState};

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
        let builder = builder.on(on_window_event).on(on_draw).on(on_frame_end);

        let attrs = self.attrs;
        builder.add_plugin_with(move |platform: &mut App| {
            let mut gfx = Gfx::new(attrs)?;
            if let Some(win) = platform.main_window() {
                gfx.init_surface(win)?;
            }

            Ok(gfx)
        })
    }
}

fn on_window_event(evt: &WindowEvent, gfx: &mut Gfx, platform: &mut App) {
    match evt.action {
        // when a new window is created let's initialize the surface for it
        WindowAction::Init => {
            gfx.init_surface(platform.window(evt.id).unwrap()).unwrap();
        }
        WindowAction::Moved { .. } => {}
        WindowAction::Resized {
            width,
            height,
            scale_factor,
        } => {
            let w = (width as f64 * scale_factor) as u32;
            let h = (height as f64 * scale_factor) as u32;
            gfx.resize(evt.id, w, h).unwrap();
        }
        WindowAction::Minimized => {}
        WindowAction::Maximized => {}
        WindowAction::FocusGained => {}
        WindowAction::FocusLost => {}
        WindowAction::Close => {}
    }
}

fn on_draw<S: GKState + 'static>(
    evt: &gk_sys::event::DrawEvent,
    gfx: &mut Gfx,
    events: &mut EventQueue<S>,
) {
    let window_id = evt.window_id;

    match gfx.raw.surfaces.get(&window_id) {
        Some(surface) => {
            gfx.current_frame = Some(window_id);
        }
        None => {
            log::warn!(
                "Cannot find a surface for window {:?}. Skipping setting it as current frame event.",
                window_id
            );
        }
    }
}

fn on_frame_end(_: &gk_sys::event::FrameEndEvent, gfx: &mut Gfx) {
    // Clean current frame info
    gfx.current_frame = None;
}
