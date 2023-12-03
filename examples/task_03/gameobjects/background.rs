use microengine::GameObject;
use microengine::KeyCode;
pub struct Background {
    color: (u8, u8, u8),
}

impl Background {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Background {
            color: (r, g, b),
        }
    }
}

impl GameObject for Background {
    fn update(&mut self, ctx: &microengine::context::Context) -> microengine::error::GameResult {
        println!("God damn! I am a background!");
        if ctx.input.kb.get_key(KeyCode::KeyA) {
            println!("i do work even finer!");
            ctx.window.close();
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
