use gamekit::app::App;
use gamekit::gfx::*;
use gamekit::math::{vec2, Mat4, Vec2};
use gamekit::prelude::*;
use gamekit::random::Rng;
use gamekit::sprite::Sprite;
use gamekit::spritebatch::SpriteBatch;
use gamekit::sys::event::{DrawEvent, UpdateEvent};
use gamekit::sys::mouse::{MouseAction, MouseEvent};
use gamekit::sys::window::GKWindow;
use gamekit::time::Time;

struct Bunny {
    sprite: Sprite,
    pos: Vec2,
    speed: Vec2,
}

#[derive(AppState)]
struct State {
    sprite: Sprite,
    sprite2: Sprite,
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

        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("./assets/bunny.png"))
            .build()?;
        let texture2 = gfx
            .create_texture()
            .from_image(include_bytes!("./assets/bunny2.png"))
            .build()?;
        let sampler = gfx.create_sampler().build()?;
        let sprite = Sprite::new(texture, sampler.clone());
        let sprite2 = Sprite::new(texture2, sampler);

        let mut batch = SpriteBatch::new(projection, gfx)?;
        let rng = Rng::new();
        let mut s = Self {
            sprite,
            sprite2,
            batch,
            bunnies: vec![],
            rng,
        };
        // s.spawn(100);
        Ok(s)
    }

    fn spawn(&mut self, n: u32) {
        (0..n).for_each(|_| {
            self.bunnies.push(Bunny {
                sprite: self.sprite.clone(),
                pos: Vec2::ZERO,
                speed: vec2(self.rng.range(0.0..10.0), self.rng.range(-5.0..5.0)),
            })
        });

        std::mem::swap(&mut self.sprite, &mut self.sprite2);
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
                    b.speed.y -= self.rng.range(0.0..6.0);
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
        .on(on_mouse_event)
        .on(on_update_event)
        .on(on_draw_update)
        .build()
}

fn on_mouse_event(evt: &MouseEvent, state: &mut State) {
    match evt.action {
        MouseAction::ButtonPressed { .. } => {
            state.spawn(2000);
        }
        _ => {}
    }
}

fn on_update_event(_: &UpdateEvent, app: &mut App, time: &mut Time, state: &mut State) {
    app.main_window().unwrap().set_title(&format!(
        "Bunny: {} - Fps: {:.2}",
        state.bunnies.len(),
        time.fps()
    ));

    state.update();
}

fn on_draw_update(evt: &DrawEvent, gfx: &mut Gfx, state: &mut State) {
    let mut frame = gfx.create_frame(evt.window_id).unwrap();

    let mut renderer = evt.create_renderer();
    renderer.clear(Some(Color::rgb(0.1, 0.2, 0.3)), None, None);
    gfx.render(&mut frame, &renderer).unwrap();

    state.bunnies.iter().for_each(|bunny| {
        state.batch.draw(&bunny.sprite, bunny.pos);
    });
    state.batch.flush(gfx, &mut frame).unwrap();
    state.batch.reset();

    gfx.present(frame).unwrap();
}
