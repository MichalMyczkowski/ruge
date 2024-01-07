use microengine::prelude::*;
use crate::config::debug;


pub struct Profiler {
    frames: usize,
    last_time_stamp: f64,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            frames: 0,
            last_time_stamp: 0.0,
        }
    }
}

impl GameObject for Profiler {
    fn start(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        self.last_time_stamp = ctx.time.get_timestamp();
        Ok(())
    }
    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        if !debug() {
            return Ok(());
        }
        
        let mut delta = ctx.time.get_timestamp() - self.last_time_stamp;
        delta -= 1.0;
        if delta > 0.0 {
            self.last_time_stamp = ctx.time.get_timestamp() - delta;
            println!("Running at: {} FPS", self.frames);
            self.frames = 0;
        }
        self.frames += 1;
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
