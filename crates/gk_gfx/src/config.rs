use crate::gfx::Gfx;
use crate::{DrawFrame, GfxAttributes};
use gk_app::App;
use gk_sys::window::{GKWindow, WindowEvent, WindowEventId};
use gk_sys::{AppBuilder, BuildConfig, EventQueue, GKState};
use std::thread::current;

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
        Ok(builder.add_plugin_with(move |platform: &mut App| {
            let mut gfx = Gfx::new(attrs)?;
            if let Some(win) = platform.main_window() {
                let _ = gfx.init_surface(win)?;
            }

            Ok(gfx)
        })?)
    }
}

fn on_window_event(evt: &WindowEvent, gfx: &mut Gfx, platform: &mut App) {
    match evt.event {
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
            gfx.resize(evt.id, w, h).unwrap();
        }
        WindowEventId::Minimized => {}
        WindowEventId::Maximized => {}
        WindowEventId::FocusGained => {}
        WindowEventId::FocusLost => {}
        WindowEventId::Close => {}
    }
}

fn on_draw<S: GKState + 'static>(
    evt: &gk_sys::event::DrawRequest,
    gfx: &mut Gfx,
    events: &mut EventQueue<S>,
) {
    let window_id = evt.window_id;

    match gfx.raw.surfaces.get(&window_id) {
        Some(surface) => {
            let width = surface.config.width;
            let height = surface.config.height;
            let frame = DrawFrame {
                window_id,
                width,
                height,
            };
            gfx.current_frame = Some(frame);
            events.queue(frame);
        }
        None => {
            log::warn!(
                "Cannot find a surface for window {:?}. Skipping DrawFrame event.",
                window_id
            );
        }
    }
}

fn on_frame_end(_: &gk_sys::event::FrameEnd, gfx: &mut Gfx) {
    // Clean current frame info
    gfx.current_frame = None;
}
