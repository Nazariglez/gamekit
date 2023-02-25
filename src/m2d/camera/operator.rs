use notan::math::{vec2, Rect, Vec2};

use crate::tween;

use super::Camera;

pub struct CameraOperator {
    pub camera: Camera,
    pub style: CameraStyle,
    target_pos: Vec2,
    target_zoom: f32,
}

impl CameraOperator {
    pub fn new(camera: Camera) -> Self {
        let style = CameraStyle {
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
            style,
            target_pos: Vec2::ZERO,
            target_zoom: 1.0,
        }
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.target_pos = vec2(x, y);
    }

    pub fn zoom_in(&mut self, factor: f32) {
        self.target_zoom -= factor;
    }

    pub fn zoom_out(&mut self, factor: f32) {
        self.target_zoom += factor;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.target_zoom = zoom;
    }

    pub fn shake(&mut self, pixels: f32, time: f32) {}

    pub fn update(&mut self, delta: f32) {
        let camera_pos = self.camera.position();
        if camera_pos == self.target_pos {
            return;
        }

        let x = if self.style.axis_x.lock {
            camera_pos.x
        } else {
            self.target_pos.x
        };

        let y = if self.style.axis_y.lock {
            camera_pos.y
        } else {
            self.target_pos.y
        };

        self.camera
            .set_position(self.target_pos.x, self.target_pos.y);

        // check limits
        if let Some(limit) = self.style.limit {
            let camera_bounds = self.camera.bounds();
            let min_x_diff = limit.min_x() - camera_bounds.min_x();
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
                self.target_pos += pos_offset;
                self.camera
                    .set_position(self.target_pos.x, self.target_pos.y);
            }
        }

        self.camera.update();
    }
    // pub fn rotate
}

pub struct CameraAxis {
    lock: bool,
    lag: f32,
    easing: tween::EaseFn,
}

impl Default for CameraAxis {
    fn default() -> Self {
        Self {
            lock: false,
            lag: 0.0,
            easing: tween::LINEAR,
        }
    }
}

#[derive(Default)]
pub struct CameraStyle {
    limit: Option<Rect>,
    axis_x: CameraAxis,
    axis_y: CameraAxis,
}

// directives? toip down? lock horizontal? limits?
