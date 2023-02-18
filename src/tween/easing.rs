#![allow(unused)]
use std::ops::{Add, Mul, Sub};

pub type EaseFn = fn(f32) -> f32;

pub fn interpolate<T>(from: T, to: T, total_time: f32, elapsed_time: f32, easing: EaseFn) -> T
where
    T: Copy + Mul<f32, Output = T> + Mul<T, Output = T> + Add<Output = T> + Sub<Output = T> + Sized,
{
    from + ((to - from) * easing(elapsed_time / total_time))
}

const LINEAR: EaseFn = |t: f32| -> f32 { t };

const IN_QUAD: EaseFn = |t: f32| -> f32 { t * t };

const OUT_QUAD: EaseFn = |t: f32| -> f32 { t * (2.0 - t) };

const IN_OUT_QUAD: EaseFn = |t: f32| -> f32 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t;
    }

    t -= 1.0;

    -0.5 * (t * (t - 2.0) - 1.0)
};

const IN_CUBIC: EaseFn = |t: f32| -> f32 { t * t * t };

const OUT_CUBIC: EaseFn = |t: f32| -> f32 { IN_CUBIC(t - 1.0) + 1.0 };

const IN_OUT_CUBIC: EaseFn = |t: f32| -> f32 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t;
    }

    t -= 2.0;
    0.5 * (t * t * t + 2.0)
};

const IN_QUART: EaseFn = |t: f32| -> f32 { t * t * t * t };

const OUT_QUART: EaseFn = |t: f32| -> f32 {
    let t = t - 1.0;
    1.0 - (t * t * t * t)
};

const IN_OUT_QUART: EaseFn = |t: f32| -> f32 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t * t;
    }
    t -= 2.0;
    -0.5 * (t * t * t * t - 2.0)
};

const IN_QUINT: EaseFn = |t: f32| -> f32 { t * t * t * t * t };

const OUT_QUINT: EaseFn = |t: f32| -> f32 {
    let t = t - 1.0;
    t * t * t * t * t + 1.0
};

const IN_OUT_QUINT: EaseFn = |t: f32| -> f32 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t * t * t;
    }
    t -= 2.0;
    0.5 * (t * t * t * t * t + 2.0)
};

const IN_SINE: EaseFn = |t: f32| -> f32 { 1.0 - ((t * std::f32::consts::PI) / 2.0).cos() };

const OUT_SINE: EaseFn = |t: f32| -> f32 { (t * std::f32::consts::PI / 2.0).sin() };

const IN_OUT_SINE: EaseFn = |t: f32| -> f32 { 0.5 * (1.0 - (std::f32::consts::PI * t).cos()) };

const IN_EXPO: EaseFn = |t: f32| -> f32 {
    if t == 0.0 {
        0.0
    } else {
        (1024.0f32).powf(t - 1.0)
    }
};

const OUT_EXPO: EaseFn = |t: f32| -> f32 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - (2.0f32).powf(-10.0 * t)
    }
};

const IN_OUT_EXPO: EaseFn = |t: f32| -> f32 {
    if t == 0.0 {
        return 0.0;
    }

    if t == 1.0 {
        return 1.0;
    }

    let t = t * 2.0;
    if t < 1.0 {
        return 0.5 * (1024f32).powf(t - 1.0);
    }

    0.5 * (-(2.0f32).powf(-10.0 * (t - 1.0)) + 2.0)
};

const IN_CIRC: EaseFn = |t: f32| -> f32 { 1.0 - (1.0 - t * t).sqrt() };

const OUT_CIRC: EaseFn = |t: f32| -> f32 {
    let t = t - 1.0;
    (1.0 - (t * t)).sqrt()
};

const IN_OUT_CIRC: EaseFn = |t: f32| -> f32 {
    let t = t * 2.0;
    if t < 1.0 {
        return -0.5 * ((1.0 - t * t).sqrt() - 1.0);
    }
    0.5 * ((1.0 - (t - 2.0) * (t - 2.0)).sqrt() + 1.0)
};

const IN_ELASTIC: EaseFn = |t: f32| -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let a = 1.0;
    let p = 0.4;
    let s = p / 4.0;

    -(a * (2.0f32).powf(10.0 * (t - 1.0))
        * (((t - 1.0) - s) * (2.0 * std::f32::consts::PI) / p).sin())
};

const OUT_ELASTIC: EaseFn = |t: f32| -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let a = 1.0;
    let p = 0.4;
    let s = p / 4.0;

    (a * (2.0f32).powf(-10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() + 1.0)
};

const IN_OUT_ELASTIC: EaseFn = |t: f32| -> f32 {
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let a = 1.0;
    let p = 0.4;
    let s = p * (1.0f32 / a).asin() / (2.0 * std::f32::consts::PI);

    let t = t * 2.0;
    if t < 1.0 {
        -0.5 * (a
            * (2.0f32).powf(10.0 * (t - 1.0))
            * (((t - 1.0) - s) * (2.0 * std::f32::consts::PI) / p).sin())
    } else {
        a * (2.0f32).powf(-10.0 * (t - 1.0))
            * (((t - 1.0) - s) * (2.0 * std::f32::consts::PI) / p).sin()
            * 0.5
            + 1.0
    }
};
const IN_BACK: EaseFn = |t: f32| -> f32 {
    let m = 1.70158;
    t * t * ((m + 1.0) * t - m)
};

const OUT_BACK: EaseFn = |t: f32| -> f32 {
    let t = t - 1.0;
    let m = 1.70158;
    t * t * ((m + 1.0) * t + m) + 1.0
};

const IN_OUT_BACK: EaseFn = |t: f32| -> f32 {
    let m = 1.70158;
    let s = m * 1.525;
    let t = t * 2.0;
    if t < 1.0 {
        0.5 * (t * t * ((s + 1.0) * t - s))
    } else {
        0.5 * ((t - 2.0) * (t - 2.0) * ((s + 1.0) * (t - 2.0) + s) + 2.0)
    }
};

const IN_BOUNCE: EaseFn = |t: f32| -> f32 { 1.0 - OUT_BOUNCE(1.0 - t) };

const OUT_BOUNCE: EaseFn = |t: f32| -> f32 {
    let m = 2.75;
    let m1 = 7.5625;
    if t < (1.0 / m) {
        m1 * t * t
    } else if t < (2.0 / m) {
        let t = (t - (1.5 / m));
        m1 * t * t + 0.75
    } else if t < (2.5 / m) {
        let t = (t - (2.25 / m));
        m1 * t * t + 0.9375
    } else {
        let t = t - (2.625 / m);
        m1 * t * t + 0.984375
    }
};

const IN_OUT_BOUNCE: EaseFn = |t: f32| -> f32 {
    if t < 0.5 {
        IN_BOUNCE(t * 2.0) * 0.5
    } else {
        OUT_BOUNCE(t * 2.0 - 1.0) * 0.5 + 0.5
    }
};

#[cfg(test)]
mod test {
    use super::*;
    use notan::math::Vec2;

    #[test]
    fn test_linear_interpolate_0() {
        let from = 0.0;
        let to = 100.0;
        let total_time = 10.0;
        let elapsed_time = 0.0;
        let value = interpolate(from, to, total_time, elapsed_time, LINEAR);
        assert_eq!(value, 0.0)
    }

    #[test]
    fn test_linear_interpolate_05() {
        let from = 0.0;
        let to = 100.0;
        let total_time = 10.0;
        let elapsed_time = 5.0;
        let value = interpolate(from, to, total_time, elapsed_time, LINEAR);
        assert_eq!(value, 50.0)
    }

    #[test]
    fn test_linear_interpolate_1() {
        let from = 0.0;
        let to = 100.0;
        let total_time = 10.0;
        let elapsed_time = 10.0;
        let value = interpolate(from, to, total_time, elapsed_time, LINEAR);
        assert_eq!(value, 100.0)
    }

    #[test]
    fn test_vec2_interpolate_0() {
        let from = Vec2::ZERO;
        let to = Vec2::splat(100.0);
        let total_time = 10.0;
        let elapsed_time = 0.0;
        let value = interpolate(from, to, total_time, elapsed_time, LINEAR);
        assert_eq!(value, Vec2::ZERO)
    }

    #[test]
    fn test_vec2_interpolate_05() {
        let from = Vec2::ZERO;
        let to = Vec2::splat(100.0);
        let total_time = 10.0;
        let elapsed_time = 5.0;
        let value = interpolate(from, to, total_time, elapsed_time, LINEAR);
        assert_eq!(value, Vec2::splat(50.0))
    }

    #[test]
    fn test_vec2_interpolate_1() {
        let from = Vec2::ZERO;
        let to = Vec2::splat(100.0);
        let total_time = 10.0;
        let elapsed_time = 10.0;
        let value = interpolate(from, to, total_time, elapsed_time, LINEAR);
        assert_eq!(value, Vec2::splat(100.0))
    }
}
