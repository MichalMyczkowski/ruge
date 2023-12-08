/// Struct responsible for calculating and providing
/// all data related to loop timing
pub struct Timer {
    pub(crate) delta_time: f64,
    pub(crate) fixed_time_step: f64,
    pub(crate) fps: u32,
    loop_start_time: f64,
    time_since_last_fixed_update: f64,
}

pub(crate) trait GetTime {
    fn get_timestamp(&self) -> f64;
}

impl Timer {
    pub(crate) fn new(fixed_fps: usize) -> Self {
        Timer {
            delta_time: 0.0,
            fps: 0,
            loop_start_time: 0.0,
            /// default is 50 times per second
            fixed_time_step: 1.0 / (fixed_fps as f64),
            time_since_last_fixed_update: 0.0,
        }
    }

    /// Marks start of loop and saves the timestamp
    pub(crate) fn loop_start(&mut self, t: &dyn GetTime) {
        self.loop_start_time = t.get_timestamp();
    }

    /// Marks end of loop and calculates delta_time
    pub(crate) fn loop_end(&mut self, t: &dyn GetTime) {
        self.delta_time = t.get_timestamp() - self.loop_start_time;
        self.time_since_last_fixed_update += self.delta_time;
    }

    /// Calculates how many fixed_update steps should be performed during next loop
    /// and changes it's internal state accordingly
    pub(crate) fn get_fixed_steps(&mut self) -> usize {
        if self.time_since_last_fixed_update > self.fixed_time_step {
            let x = self.time_since_last_fixed_update / self.fixed_time_step;
            self.time_since_last_fixed_update -= self.fixed_time_step * x.floor();
            x.floor() as usize
        } else {
            0
        }
    }
    
    /// Returns frame start timestamp.
    pub fn get_timestamp(&self) -> f64 {
        self.loop_start_time
    }

    /// Returns delta_time, this is the only method GameObjects need
    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer::new(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut COUNTER: usize = 0;

    fn reset_counter() {
        unsafe {
            COUNTER = 0;
        }
    }

    fn count_up() -> f64 {
        unsafe {
            COUNTER += 1;
            (COUNTER - 1) as f64
        }
    }

    struct FakeTimer;
    impl FakeTimer {
        pub fn new() -> Self {
            reset_counter();
            FakeTimer {}
        }
    }
    impl GetTime for FakeTimer {
        fn get_timestamp(&self) -> f64 {
            count_up()
        }
    }

    #[test]
    fn updates_delta_time() {
        let mut t = Timer::default();
        let get_time = FakeTimer::new();
        t.loop_start(&get_time);
        t.loop_end(&get_time);
        assert_eq!(t.delta_time(), 1.0);
    }

    #[test]
    fn resets_on_each_loop_start() {
        let mut t = Timer::default();
        let get_time = FakeTimer::new();
        for _ in 0..10 {
            t.loop_start(&get_time);
            t.loop_end(&get_time);
            assert_eq!(t.delta_time(), 1.0);
        }
    }

    #[test]
    fn calculates_fixed_update_count() {
        let mut t = Timer::default();
        let get_time = FakeTimer::new();
        t.loop_start(&get_time);
        t.loop_end(&get_time);
        assert_eq!(t.get_fixed_steps(), (1.0 / t.fixed_time_step) as usize);
    }

    #[test]
    fn resets_fixed_update_count_each_time_is_asked_about_it() {
        let mut t = Timer::default();
        let get_time = FakeTimer::new();
        for _ in 0..10 {
            t.loop_start(&get_time);
            t.loop_end(&get_time);
            assert_eq!(t.get_fixed_steps(), (1.0 / t.fixed_time_step) as usize);
        }
    }
}

impl GetTime for Timer {
    /// [!WARNING]
    /// This is not a super accurate timestamp, but should be good enough for game purposes
    fn get_timestamp(&self) -> f64 {
        self.loop_start_time
    }
}
