use notan::math::{vec2, vec3, Mat3, Mat4, Vec2};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum CameraMode {
    #[default]
    Basic,
    Fill(Vec2),
    AspectFit(Vec2),
    AspectFill(Vec2),
}

impl CameraMode {
    fn work_size(&self) -> Option<Vec2> {
        Some(match self {
            CameraMode::Basic => return None,
            CameraMode::Fill(w) => *w,
            CameraMode::AspectFit(w) => *w,
            CameraMode::AspectFill(w) => *w,
        })
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum CameraStyle {
    #[default]
    LockOn,
    Platformer,
    TopDown,
    TopDownTight,
}

pub struct Camera {
    dirty_projection: bool,
    dirty_transform: bool,
    position: Vec2,
    rotation: f32,
    scale: Vec2,
    size: Vec2,

    projection: Mat4,
    ratio: Vec2,
    transform: Mat3,

    mode: CameraMode,
    style: CameraStyle,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            size: vec2(1.0, 1.0),
            position: vec2(0.0, 0.0),
            scale: vec2(1.0, 1.0),
            rotation: 0.0,
            dirty_projection: true,
            dirty_transform: true,
            projection: Mat4::IDENTITY,
            ratio: vec2(1.0, 1.0),
            transform: Mat3::IDENTITY,
            mode: CameraMode::Basic,
            style: CameraStyle::LockOn,
        }
    }
}

impl Camera {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }

    pub fn set_mode(&mut self, mode: CameraMode) {
        if self.mode != mode {
            self.mode = mode;
            self.dirty_projection = true;
        }
    }

    pub fn mode(&self) -> CameraMode {
        self.mode
    }

    pub fn set_style(&mut self, style: CameraStyle) {
        if self.style != style {
            self.style = style;
            self.dirty_projection = true;
        }
    }

    pub fn style(&self) -> CameraStyle {
        self.style
    }

    pub fn set_size(&mut self, x: f32, y: f32) {
        let size = vec2(x, y);
        if self.size != size {
            self.size = size;
            self.dirty_projection = true;
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        let pos = vec2(x, y);
        if self.position != pos {
            self.position = pos;
            self.dirty_transform = true;
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_rotation(&mut self, angle: f32) {
        if self.rotation != angle {
            self.rotation = angle;
            self.dirty_transform = true;
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        let scale = vec2(x, y);
        if self.scale != scale {
            self.scale = scale;
            self.dirty_transform = true;
        }
    }

    pub fn scale(&self) -> Vec2 {
        self.scale
    }

    pub fn set_zoom(&mut self, scale: f32) {
        self.set_scale(scale, scale);
    }

    pub fn zoom(&self) -> f32 {
        self.scale.x
    }

    pub fn transform(&self) -> Mat3 {
        self.transform
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }

    pub fn update(&mut self) {
        if self.dirty_projection {
            self.dirty_projection = false;
            self.calculate_projection();
            return;
        }

        if self.dirty_transform {
            self.dirty_transform = false;
            self.calculate_transform();
        }
    }

    fn calculate_projection(&mut self) {
        match self.mode {
            CameraMode::Basic => self.calculate_ortho_projection(),
            CameraMode::Fill(work_size) => self.calculate_fill_projection(work_size),
            CameraMode::AspectFit(work_size) => self.calculate_aspect_fit_projection(work_size),
            CameraMode::AspectFill(work_size) => self.calculate_aspect_fill_projection(work_size),
        }
    }

    fn calculate_transform(&mut self) {
        let translation = Mat3::from_translation(-self.position);
        let rotation = Mat3::from_angle(self.rotation);
        let scale = Mat3::from_scale(self.scale);
        let transform = rotation * scale * translation;
        self.transform = transform;
    }

    fn calculate_ortho_projection(&mut self) {
        let (projection, ratio) = calculate_ortho_projection(self.size);
        self.projection = projection;
        self.ratio = ratio;
    }

    fn calculate_fill_projection(&mut self, work_size: Vec2) {
        let (projection, ratio) = calculate_fill_projection(self.size, work_size);
        self.projection = projection;
        self.ratio = ratio;
    }

    fn calculate_aspect_fit_projection(&mut self, work_size: Vec2) {
        let (projection, ratio) = calculate_aspect_fit_projection(self.size, work_size);
        self.projection = projection;
        self.ratio = ratio;
    }

    fn calculate_aspect_fill_projection(&mut self, work_size: Vec2) {
        let (projection, ratio) = calculate_aspect_fill_projection(self.size, work_size);
        self.projection = projection;
        self.ratio = ratio;
    }
}

fn calculate_ortho_projection(win_size: Vec2) -> (Mat4, Vec2) {
    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let pos = win_size * 0.5;
    let position = Mat4::from_translation(vec3(pos.x, pos.y, 1.0));
    let final_projection = projection * position;
    (final_projection, vec2(1.0, 1.0))
}

fn calculate_fill_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, Vec2) {
    let ratio = vec2(win_size.x / work_size.x, win_size.y / work_size.y);
    (Mat4::IDENTITY, ratio)
}

fn calculate_aspect_fit_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, Vec2) {
    let ratio = (win_size.x / work_size.x).min(win_size.y / work_size.y);
    let ratio = Vec2::splat(ratio);
    let scale = Mat4::from_scale(vec3(ratio.x, ratio.y, 1.0));
    let pos = win_size * 0.5;
    let position = vec3(pos.x, pos.y, 1.0);
    let translation = Mat4::from_translation(position);
    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let final_projection = projection * translation * scale;
    (final_projection, ratio)
}

fn calculate_aspect_fill_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, Vec2) {
    let ratio = (win_size.x / work_size.x).max(win_size.y / work_size.y);
    let ratio = Vec2::splat(ratio);
    (Mat4::IDENTITY, ratio)
}
