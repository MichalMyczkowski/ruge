use microengine::GameObject;
use microengine::KeyCode;
pub struct Labirynth {
    size: usize,
}

impl Labirynth {
    pub fn new(size: usize) -> Self {
        Labirynth {
            size,
        }
    }
}

impl GameObject for Labirynth {
    fn update(&mut self, ctx: &microengine::context::Context) -> microengine::error::GameResult {
        println!("God damn! I am working fine!");
        if ctx.input.kb.get_key(KeyCode::KeyA) {
            println!("works just fine!");
            ctx.window.close();
        }
        Ok(())
    }

    fn finished_update(&mut self, _ctx: &microengine::context::Context) -> microengine::error::GameResult<bool> {
        println!("hi my name is finished!");
        Ok(true)
    }
    

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
