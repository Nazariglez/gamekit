use notan::math::{vec2, vec3, Mat3, Mat4, Rect, Vec2};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ScreenMode {
    #[default]
    Basic,
    Fill(Vec2),
    AspectFit(Vec2),
    AspectFill(Vec2),
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum FollowStyle {
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
    local_bounds: Rect,

    mode: ScreenMode,
    style: FollowStyle,
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
            mode: ScreenMode::Basic,
            style: FollowStyle::LockOn,
            local_bounds: Rect::default(),
        }
    }
}

impl Camera {
    pub fn new(size: Vec2) -> Self {
        let local_bounds = Rect {
            x: 0.0,
            y: 0.0,
            width: size.x,
            height: size.y,
        };

        Self {
            size,
            local_bounds,
            ..Default::default()
        }
    }

    pub fn bounds(&self) -> Rect {
        self.local_bounds
    }

    pub fn set_screen_mode(&mut self, mode: ScreenMode) {
        if self.mode != mode {
            self.mode = mode;
            self.dirty_projection = true;
        }
    }

    pub fn screen_mode(&self) -> ScreenMode {
        self.mode
    }

    pub fn set_follow_style(&mut self, style: FollowStyle) {
        if self.style != style {
            self.style = style;
            self.dirty_projection = true;
        }
    }

    pub fn follow_style(&self) -> FollowStyle {
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
        // Check if we need to recalculate bounds after the
        // projection and transform are updated
        let dirty_bounds = self.dirty_projection || self.dirty_transform;

        if self.dirty_projection {
            self.dirty_projection = false;
            self.calculate_projection();
        }

        if self.dirty_transform {
            self.dirty_transform = false;
            self.calculate_transform();
        }

        if dirty_bounds {
            self.calculate_bounds();
        }
    }

    pub fn resolution(&self) -> Vec2 {
        match self.mode {
            ScreenMode::Basic => self.size,
            ScreenMode::Fill(r) => r,
            ScreenMode::AspectFit(r) => r,
            ScreenMode::AspectFill(r) => r,
        }
    }

    fn calculate_projection(&mut self) {
        let (projection, ratio) = match self.mode {
            ScreenMode::Basic => calculate_ortho_projection(self.size),
            ScreenMode::Fill(work_size) => calculate_fill_projection(self.size, work_size),
            ScreenMode::AspectFit(work_size) => {
                calculate_aspect_fit_projection(self.size, work_size)
            }
            ScreenMode::AspectFill(work_size) => {
                calculate_aspect_fill_projection(self.size, work_size)
            }
        };

        self.projection = projection;
        self.ratio = ratio;
    }

    fn calculate_transform(&mut self) {
        let translation = Mat3::from_translation(-self.position);
        let rotation = Mat3::from_angle(self.rotation);
        let scale = Mat3::from_scale(self.scale);
        let transform = rotation * scale * translation;
        self.transform = transform;
    }

    fn calculate_bounds(&mut self) {
        let size = self.size / (self.ratio * self.scale);
        let pos = self.position - (size * 0.5);
        self.local_bounds = Rect {
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        };
    }
}

fn calculate_ortho_projection(win_size: Vec2) -> (Mat4, Vec2) {
    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let pos = win_size * 0.5;
    let position = Mat4::from_translation(vec3(pos.x, pos.y, 1.0));
    let final_projection = projection * position;
    (final_projection, vec2(1.0, 1.0))
}

fn calculate_scaled_projection(win_size: Vec2, ratio: Vec2) -> Mat4 {
    let scale = Mat4::from_scale(vec3(ratio.x, ratio.y, 1.0));
    let pos = win_size * 0.5;
    let position = vec3(pos.x, pos.y, 1.0);
    let translation = Mat4::from_translation(position);
    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let final_projection = projection * translation * scale;
    final_projection
}

fn calculate_fill_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, Vec2) {
    let ratio = vec2(win_size.x / work_size.x, win_size.y / work_size.y);
    let projection = calculate_scaled_projection(win_size, ratio);
    (projection, ratio)
}

fn calculate_aspect_fit_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, Vec2) {
    let ratio = (win_size.x / work_size.x).min(win_size.y / work_size.y);
    let ratio = Vec2::splat(ratio);
    let projection = calculate_scaled_projection(win_size, ratio);
    (projection, ratio)
}

fn calculate_aspect_fill_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, Vec2) {
    let ratio = (win_size.x / work_size.x).max(win_size.y / work_size.y);
    let ratio = Vec2::splat(ratio);
    let projection = calculate_scaled_projection(win_size, ratio);
    (projection, ratio)
}
