use gamekit::app::App;
use gamekit::gfx::*;
use gamekit::math::{vec2, Vec2};
use gamekit::prelude::*;
use gamekit::spritebatch::SpriteBatch;
use gamekit::sys::event::DrawEvent;
use gamekit::time::Time;
use gk_math::Mat4;
use gk_sys::window::GKWindow;

#[derive(AppState)]
struct State {
    batch: SpriteBatch,
}

impl State {
    pub fn new(app: &mut App, gfx: &mut Gfx) -> Result<Self, String> {
        let mut batch = SpriteBatch::new(include_bytes!("./assets/bunny.png"), gfx)?;

        // Set camera projection
        if let Some(win) = app.main_window() {
            let (w, h) = win.size();
            let projection = Mat4::orthographic_rh_gl(0.0, w as _, h as _, 0.0, -1.0, 1.0);
            batch.set_projection(projection);
        }

        Ok(Self { batch })
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(App::config())?
        .add_config(Gfx::config().with_vsync(true))?
        .add_config(Time::config())?
        .on(
            |evt: &DrawEvent, gfx: &mut Gfx, time: &mut Time, state: &mut State| {
                state.batch.draw(vec2(10.0, 10.0));
                // state.batch.draw(vec2(-0.2, -0.2));
                // state.batch.draw(vec2(0.2, 0.2));
                // state.batch.draw(vec2(0.4, 0.4));
                state.batch.flush(gfx).unwrap();
                state.batch.reset();
            },
        )
        .build()
}
