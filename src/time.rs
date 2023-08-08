use crate::utils::RingBuffer;
use gk_app::{event, AppBuilder, BuildConfig, GKState, Plugin};
use std::time::{Duration, Instant};

/// Measure Application times
#[derive(Debug, Clone)]
pub struct Time {
    init_time: Instant,
    last_time: Option<Instant>,
    delta: Duration,
    delta_seconds: f32,
    elapsed: Duration,
    elapsed_time: f32,
    fps_cache: RingBuffer<f32, 600>,
    fps: f32,
}

impl Default for Time {
    fn default() -> Time {
        Time {
            init_time: Instant::now(),
            last_time: None,
            delta: Duration::from_secs(0),
            delta_seconds: 0.0,
            elapsed: Duration::from_secs(0),
            elapsed_time: 0.0,
            fps_cache: Default::default(),
            fps: 0.0,
        }
    }
}

impl Time {
    pub fn config() -> TimeConfig {
        TimeConfig::default()
    }

    #[inline]
    pub(crate) fn update(&mut self) {
        let now = Instant::now();

        if let Some(last_time) = self.last_time {
            self.delta = now - last_time;
            self.delta_seconds = self.delta.as_secs_f32();
        }

        self.last_time = Some(now);

        self.elapsed = now - self.init_time;
        self.elapsed_time = self.elapsed.as_secs_f32();

        self.fps_cache.push(self.delta_seconds);
        self.fps = 1.0 / (self.fps_cache.iter().sum::<f32>() / self.fps_cache.len() as f32);
    }

    /// Average frames per second (calculated using the last 60 frames)
    #[inline]
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// Delta time between frames
    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    /// Delta time between frames in seconds
    #[inline]
    pub fn delta_f32(&self) -> f32 {
        self.delta_seconds
    }

    /// Elapsed time since application's init
    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Elapsed time since application's init in seconds
    #[inline]
    pub fn elapsed_f32(&self) -> f32 {
        self.elapsed_time
    }

    /// Application's init time
    #[inline]
    pub fn init_time(&self) -> Instant {
        self.init_time
    }

    /// Last frame time
    #[inline]
    pub fn last_time(&self) -> Option<Instant> {
        self.last_time
    }
}

impl Plugin for Time {}

#[derive(Debug, Default, Copy, Clone)]
pub struct TimeConfig;

impl<S: GKState + 'static> BuildConfig<S> for TimeConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let builder = builder.on(|_: &event::FrameStart, time: &mut Time| time.update());
        Ok(builder.add_plugin(Time::default()))
    }
}
