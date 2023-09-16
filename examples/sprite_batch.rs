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
        let projection = app.main_window().map_or(Mat4::IDENTITY, |win| {
            let (w, h) = win.size();
            Mat4::orthographic_rh_gl(0.0, w as _, h as _, 0.0, -1.0, 1.0)
        });

        let mut batch = SpriteBatch::new(include_bytes!("./assets/bunny.png"), projection, gfx)?;
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
                state.batch.draw(vec2(100.0, 100.0));
                state.batch.draw(vec2(200.0, 100.0));
                state.batch.draw(vec2(300.0, 100.0));
                state.batch.draw(vec2(400.0, 100.0));
                state.batch.flush(gfx).unwrap();
                state.batch.reset();
            },
        )
        .build()
}
