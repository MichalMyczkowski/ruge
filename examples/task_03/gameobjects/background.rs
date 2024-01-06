use crate::drawable::BackgroundDrawable;
use glm::Vec2;
use microengine::GameObject;

pub struct Background {
    drawable: BackgroundDrawable,
}

impl Background {
    pub fn new(position: Vec2, size: f32) -> Self {
        Background {
            drawable: BackgroundDrawable::new(size, position),
        }
    }
}

impl GameObject for Background {
    fn start(&mut self, _ctx: &microengine::context::Context, _scene: &microengine::Scene) -> microengine::error::GameResult {
        unsafe {
        gl::Disable(gl::DEPTH_TEST);
        }
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &microengine::context::Context,
        _scene: &microengine::Scene,
    ) -> microengine::error::GameResult {
        self.drawable.draw(
            ctx.window.aspect_ratio() as f32,
            ctx.time.get_timestamp() as f32,
        );
        Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
