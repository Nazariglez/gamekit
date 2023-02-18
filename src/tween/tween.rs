use super::EaseFn;

pub struct Tween {
    easing: EaseFn,
}

impl Tween {
    pub fn new(easing: EaseFn) -> Self {
        Self { easing }
    }
}
