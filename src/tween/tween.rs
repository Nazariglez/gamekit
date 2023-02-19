use super::{interpolate, EaseFn, Interpolable, LINEAR};

#[derive(Clone, Copy)]
enum RepeatMode {
    Never,
    Forever,
    Times(u32),
}

#[derive(Clone, Copy)]
enum State {
    Idle,
    Started,
    Ended,
}

#[derive(Clone, Copy)]
pub struct Tween<T: Interpolable> {
    from: T,
    to: T,
    repeat_mode: RepeatMode,
    state: State,
    easing: EaseFn,
    time: f32,
    delay: f32,
    yoyo_enabled: bool,
    yoyo_back: bool,
    elapsed_time: f32,
    elapsed_delay: f32,
    repeated: u32,
    value: T,
}

impl<T: Interpolable> Tween<T> {
    pub fn new(from: T, to: T, time: f32) -> Self {
        Self {
            from,
            to,
            time,
            state: State::Idle,
            easing: LINEAR,
            delay: 0.0,
            repeat_mode: RepeatMode::Never,
            yoyo_enabled: false,
            yoyo_back: false,
            elapsed_time: 0.0,
            elapsed_delay: 0.0,
            repeated: 0,
            value: from,
        }
    }

    pub fn tick(&mut self, delta: f32) {
        if !can_update(&self) {
            return;
        }

        if self.elapsed_delay < self.delay {
            self.elapsed_delay += delta;
            return;
        }

        let time = if self.yoyo_enabled {
            self.time * 0.5
        } else {
            self.time
        };

        if self.elapsed_time < time {
            let current_time = self.elapsed_time + delta;
            let did_finish = current_time >= time;

            self.elapsed_time = if did_finish { time } else { current_time };

            self.value = interpolate(self.from, self.to, time, self.elapsed_time, self.easing);

            // TODO total_time??

            if did_finish {
                if self.yoyo_enabled && !self.yoyo_back {
                    self.yoyo_back = true;
                    std::mem::swap(&mut self.from, &mut self.to);
                    self.elapsed_time = 0.0;
                    return;
                }

                let repeat = match self.repeat_mode {
                    RepeatMode::Forever => true,
                    RepeatMode::Times(times) => self.repeated < times,
                    _ => false,
                };

                if repeat {
                    self.repeated += 1;
                    self.elapsed_time = 0.0;

                    if self.yoyo_enabled && self.yoyo_back {
                        self.yoyo_back = false;
                        std::mem::swap(&mut self.from, &mut self.to);
                    }

                    return;
                }

                self.state = State::Ended;
            }
        }
    }

    pub fn start(&mut self) {
        match self.state {
            State::Idle => self.state = State::Started,
            State::Ended => {
                self.reset();
                self.state = State::Started;
            }
            _ => {}
        }
    }

    pub fn stop(&mut self) {
        if matches!(self.state, State::Started) {
            self.state = State::Idle;
        }
    }

    pub fn reset(&mut self) {
        self.state = State::Idle;
        self.elapsed_time = 0.0;
        self.elapsed_delay = 0.0;
        self.repeated = 0;
    }

    pub fn set_repeat(&mut self, times: u32) {
        self.repeat_mode = RepeatMode::Times(times);
    }

    pub fn set_repeat_forever(&mut self, repeat: bool) {
        self.repeat_mode = if repeat {
            RepeatMode::Forever
        } else {
            RepeatMode::Never
        };
    }

    pub fn set_yoyo(&mut self, yoyo: bool) {
        self.yoyo_enabled = yoyo;
    }

    pub fn set_easing(&mut self, easing: EaseFn) {
        self.easing = easing;
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn running_time(&self) -> f32 {
        self.time * (self.repeated as f32) + self.elapsed_time
    }

    pub fn is_started(&self) -> bool {
        matches!(self.state, State::Started)
    }

    pub fn is_ended(&self) -> bool {
        matches!(self.state, State::Ended)
    }
}

fn can_update<T: Interpolable>(tween: &Tween<T>) -> bool {
    if !matches!(tween.state, State::Started) {
        return false;
    }

    return tween.time > 0.0;
}
