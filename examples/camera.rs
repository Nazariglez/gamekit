// use std::any::Any;
// use std::any::TypeId;
// use std::ops::Add;
// use std::ops::Mul;
// use std::ops::Sub;
//
// use gamekit::m2d::*;
// use gamekit::tween::*;
// use notan::draw::*;
// use notan::math::*;
// use notan::prelude::*;
//
// const RESOLUTION: Vec2 = Vec2::new(300.0, 300.0);
// const POS: Vec2 = Vec2::new(0.0, 0.0);
//
// #[derive(AppState)]
// struct State {
//     camera: CameraOperator,
//     entity: Vec2,
//     // tween_x: Tween<f32>,
//     // tween_y: Tween<f32>,
// }
//
// impl State {
//     fn new() -> Self {
//         let mut camera = Camera::new(vec2(800.0, 600.0));
//         // camera.set_position(400.0, 300.0);
//         camera.set_screen_mode(ScreenMode::AspectFit(RESOLUTION));
//         // let mut tween = Tween::new(POS, vec2(800.0, 600.0), 5.0);
//         // tween.set_repeat(2);
//         // tween.set_yoyo(true);
//         // tween.set_easing(IN_OUT_BOUNCE);
//
//         // let time = 5.0;
//         // let tween_x = Tween::new(0.0, 800.0, time);
//         // let mut tween_y = Tween::new(600.0, 0.0, time);
//         // tween_y.set_easing(IN_OUT_BOUNCE);
//
//         let camera = CameraOperator::new(camera);
//         Self {
//             camera,
//             entity: vec2(400.0, 300.0),
//             // tween_x,
//             // tween_y,
//         }
//     }
// }
//
// fn main() {
//     // let mut container = vec![];
//     // container.push(Tween::new(0.0, 1.0, 10.0));
//     // container.push(Tween::new(vec2(1.0, 1.0), vec2(1.0, 1.0), 10.0));
//
//     let win_conf = WindowConfig::default().resizable(true);
//     notan::init_with(State::new)
//         .add_config(win_conf)
//         .add_config(DrawConfig)
//         .update(update)
//         .draw(draw)
//         .build()
//         .unwrap();
// }
//
// fn update(app: &mut App, state: &mut State) {
//     let speed = 50.0;
//     if app.keyboard.is_down(KeyCode::A) {
//         state.entity.x -= speed * app.timer.delta_f32();
//     } else if app.keyboard.is_down(KeyCode::D) {
//         state.entity.x += speed * app.timer.delta_f32();
//     }
//
//     if app.keyboard.is_down(KeyCode::W) {
//         state.entity.y -= speed * app.timer.delta_f32();
//     } else if app.keyboard.is_down(KeyCode::S) {
//         state.entity.y += speed * app.timer.delta_f32();
//     }
//
//     let (w, h) = app.window().size();
//     state.camera.camera.set_size(w as _, h as _);
//     state.camera.look_at(state.entity.x, state.entity.y);
//     state.camera.update(app.timer.delta_f32());
// }
//
// fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
//     if app.keyboard.was_pressed(KeyCode::Q) {
//         app.exit();
//         return;
//     }
//
//     // if app.keyboard.is_down(KeyCode::J) {
//     //     let zoom = state.camera.zoom();
//     //     state.camera.set_zoom(zoom - 10.0 * app.timer.delta_f32());
//     // } else if app.keyboard.is_down(KeyCode::K) {
//     //     let zoom = state.camera.zoom();
//     //     state.camera.set_zoom(zoom + 10.0 * app.timer.delta_f32());
//     // }
//
//     // if app.keyboard.is_down(KeyCode::H) {
//     //     let rotation = state.camera.rotation();
//     //     state
//     //         .camera
//     //         .set_rotation(rotation - 10f32.to_radians() * app.timer.delta_f32());
//     // } else if app.keyboard.is_down(KeyCode::L) {
//     //     let rotation = state.camera.rotation();
//     //     state
//     //         .camera
//     //         .set_rotation(rotation + 10f32.to_radians() * app.timer.delta_f32());
//     // }
//
//     let projection = state.camera.camera.projection();
//     let transform = state.camera.camera.transform();
//
//     let mut draw = gfx.create_draw();
//     draw.set_projection(Some(projection));
//     draw.transform().push(transform);
//     draw.clear(Color::BLACK);
//
//     let start: (f32, f32) = ((RESOLUTION * 0.5 * -1.0) + state.entity).into();
//     draw.rect(start, RESOLUTION.into())
//         .stroke_color(Color::ORANGE)
//         .stroke(4.0);
//
//     draw.circle(20.0)
//         .position(state.entity.x, state.entity.y)
//         .fill_color(Color::RED)
//         .fill()
//         .stroke_color(Color::WHITE)
//         .stroke(4.0);
//
//     draw.rect((400.0, 300.0), (20.0, 20.0))
//         .color(Color::MAGENTA);
//
//     draw.line((0.0, 0.0), (800.0, 600.0)).width(10.0);
//
//     draw.line((0.0, 600.0), (800.0, 0.0)).width(10.0);
//
//     // let bounds = state.camera.bounds();
//     // draw.rect((bounds.x, bounds.y), (bounds.width, bounds.height))
//     //     .stroke_color(Color::GREEN)
//     //     .stroke(10.0);
//
//     draw.rect((0.0, 0.0), (800.0, 600.0))
//         .stroke_color(Color::GREEN)
//         .stroke(1.0);
//
//     gfx.render(&draw);
// }
