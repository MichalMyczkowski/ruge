use microengine::prelude::*;
use crate::config::debug;


pub struct Profiler {
    delta_time: f64,
    frames: usize,

}

impl Profiler {
    pub fn new() -> Self {
        Self {
            frames: 0,
            delta_time: 0.0,
        }
    }
}

impl GameObject for Profiler {
    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        if !debug() {
            return Ok(());
        }
        if self.delta_time > 1.0 {
            println!("Running at: {} FPS", self.frames);
            self.frames = 0;
            self.delta_time -= 1.0;
        }
        self.frames += 1;
        self.delta_time += ctx.time.delta_time();
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
