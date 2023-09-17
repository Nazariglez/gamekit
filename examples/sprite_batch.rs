use gamekit::app::App;
use gamekit::gfx::*;
use gamekit::math::{vec2, Vec2, Mat4};
use gamekit::prelude::*;
use gamekit::spritebatch::SpriteBatch;
use gamekit::sys::event::{DrawEvent, UpdateEvent};
use gamekit::time::Time;
use gamekit::sys::window::GKWindow;
use gamekit::sys::mouse::{MouseAction, MouseEvent};
use gamekit::random::Rng;

struct Bunny {
    pos: Vec2,
    speed: Vec2,
}

#[derive(AppState)]
struct State {
    batch: SpriteBatch,
    bunnies: Vec<Bunny>,
    rng: Rng,
}

impl State {
    pub fn new(app: &mut App, gfx: &mut Gfx) -> Result<Self, String> {
        let projection = app.main_window().map_or(Mat4::IDENTITY, |win| {
            let (w, h) = win.size();
            Mat4::orthographic_rh_gl(0.0, w as _, h as _, 0.0, -1.0, 1.0)
        });

        let mut batch = SpriteBatch::new(include_bytes!("./assets/bunny.png"), projection, gfx)?;
        let rng = Rng::new();
        Ok(Self { batch, bunnies: vec![], rng })
    }

    fn spawn(&mut self, n: u32) {
        (0..n).for_each(|_| {
            self.bunnies.push(Bunny {
                pos: Vec2::ZERO,
                speed: vec2(0.0 + self.rng.gen::<f32>() * 10.0, -5.0 + self.rng.gen::<f32>() * 5.0),
            })
        });
    }

    fn update(&mut self) {
        self.bunnies.iter_mut().for_each(|b| {
            b.pos += b.speed;
            b.speed.y += 0.75;

            if b.pos.x > 800.0 {
                b.speed.x *= -1.0;
                b.pos.x = 800.0;
            } else if b.pos.x < 0.0 {
                b.speed.x *= -1.0;
                b.pos.x = 0.0
            }

            if b.pos.y > 600.0 {
                b.speed.y *= -0.85;
                b.pos.y = 600.0;
                if self.rng.gen::<bool>() {
                    b.speed.y -= self.rng.gen::<f32>() * 6.0;
                }
            } else if b.pos.y < 0.0 {
                b.speed.y = 0.0;
                b.pos.y = 0.0;
            }
        });
    }
}

fn main() -> Result<(), String> {
    gamekit::init_with(State::new)
        .add_config(App::config())?
        .add_config(Gfx::config())?
        .add_config(Time::config())?
        .on(|evt: &MouseEvent, state: &mut State| match evt.action {
            MouseAction::ButtonPressed { .. } => {
               state.spawn(1000);
            }
            MouseAction::ButtonReleased { .. } => {}
            _ => {}
        })
        .on(
            |evt: &UpdateEvent, app: &mut App, time: &mut Time, state: &mut State| {
                app.main_window().unwrap().set_title(&format!(
                    "Bunny: {} - Fps: {:.2}",
                    state.bunnies.len(),
                    time.fps()
                ));

                state.update();
            },
        )
        .on(
            |evt: &DrawEvent, gfx: &mut Gfx, time: &mut Time, state: &mut State| {
                state.bunnies.iter().for_each(|bunny| {
                   state.batch.draw(bunny.pos);
                });
                state.batch.flush(gfx).unwrap();
                state.batch.reset();
            },
        )
        .build()
}
