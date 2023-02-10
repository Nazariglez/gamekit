use gamekit::m2d::*;
use notan::draw::*;
use notan::math::*;
use notan::prelude::*;

#[derive(AppState, Default)]
struct State {
    camera: Camera,
    entity: Vec2,
}

impl State {
    fn new() -> Self {
        let mut camera = Camera::new(vec2(800.0, 600.0));
        camera.set_mode(CameraMode::AspectFit(vec2(800.0, 600.0)));
        Self {
            camera,
            entity: vec2(0.0, 0.0),
        }
    }
}

fn main() {
    notan::init_with(State::new)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
        .unwrap();
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Q) {
        app.exit();
        return;
    }

    if app.keyboard.is_down(KeyCode::J) {
        let zoom = state.camera.zoom();
        state.camera.set_zoom(zoom - 10.0 * app.timer.delta_f32());
    } else if app.keyboard.is_down(KeyCode::K) {
        let zoom = state.camera.zoom();
        state.camera.set_zoom(zoom + 10.0 * app.timer.delta_f32());
    }

    let speed = 50.0;
    if app.keyboard.is_down(KeyCode::A) {
        state.entity.x += speed * app.timer.delta_f32();
    } else if app.keyboard.is_down(KeyCode::D) {
        state.entity.x -= speed * app.timer.delta_f32();
    }

    state.camera.set_position(state.entity.x, state.entity.y);
    state.camera.update();

    let projection = state.camera.projection();
    let transform = state.camera.transform();

    let mut draw = gfx.create_draw();
    draw.set_projection(Some(projection));
    draw.transform().push(transform);
    draw.clear(Color::BLACK);

    draw.rect((-400.0, -300.0), (800.0, 600.0))
        .stroke_color(Color::ORANGE)
        .stroke(4.0);

    draw.circle(20.0)
        .position(state.entity.x, state.entity.y)
        .fill_color(Color::RED)
        .fill()
        .stroke_color(Color::WHITE)
        .stroke(4.0);

    gfx.render(&draw);
}