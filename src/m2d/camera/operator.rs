use notan::math::{vec2, Rect, Vec2};

use super::Camera;

pub struct CameraOperator {
    pub camera: Camera,
    pub director: CameraDirector,
    target_pos: Vec2,
}

impl CameraOperator {
    pub fn new(camera: Camera) -> Self {
        let director = CameraDirector {
            limit: Some(Rect {
                x: 0.0,
                y: 0.0,
                width: 800.0,
                height: 600.0,
            }),
            ..Default::default()
        };

        Self {
            camera,
            director,
            target_pos: Vec2::ZERO,
        }
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.target_pos = vec2(x, y);
    }

    pub fn zoom_in(&mut self, factor: f32) {}
    pub fn zoom_out(&mut self, factor: f32) {}
    pub fn shake(&mut self, pixels: f32, time: f32) {}

    pub fn update(&mut self, delta: f32) {
        let mut camera_pos = self.camera.position();
        if camera_pos == self.target_pos {
            return;
        }

        println!("11111");
        dbg!(self.target_pos, camera_pos);

        self.camera
            .set_position(self.target_pos.x, self.target_pos.y);
        // self.camera.update();

        // check limits
        if let Some(limit) = self.director.limit {
            let camera_bounds = self.camera.bounds();
            let min_x_diff = dbg!(limit.min_x() - camera_bounds.min_x());
            let max_x_diff = limit.max_x() - camera_bounds.max_x();
            let min_y_diff = limit.min_y() - camera_bounds.min_y();
            let max_y_diff = limit.max_y() - camera_bounds.max_y();

            let mut pos_offset = Vec2::ZERO;
            if min_x_diff > f32::EPSILON {
                pos_offset.x = min_x_diff;
            } else if max_x_diff < -f32::EPSILON {
                pos_offset.x = max_x_diff;
            }

            if min_y_diff > f32::EPSILON {
                pos_offset.y = min_y_diff;
            } else if max_y_diff < -f32::EPSILON {
                pos_offset.y = max_y_diff;
            }

            if pos_offset != Vec2::ZERO {
                dbg!(camera_bounds);
                dbg!(pos_offset);
                self.target_pos += pos_offset;
                self.camera
                    .set_position(self.target_pos.x, self.target_pos.y);
            }
        }

        println!("22222");
        dbg!(self.target_pos, self.camera.position());
        self.camera.update();
    }
    // pub fn rotate
}

#[derive(Default)]
pub struct CameraDirector {
    limit: Option<Rect>,
    lock_x: bool,
    lock_y: bool,
}

// directives? toip down? lock horizontal? limits?
