use microengine::{GameObject, GameObjectId};
use nalgebra_glm::Vec2;

use crate::drawable::FinishDrawable;

use super::Player;

pub struct Finish {
    position: Vec2,
    player_id: Option<GameObjectId>,
    drawable: FinishDrawable,
    start_time: f32,
}

impl Finish {
    pub fn new(position: Vec2, size: f32) -> Self {
        Self {
            position,
            player_id: None,
            drawable: FinishDrawable::new(size, position),
            start_time: 0.0,
        }
    }

    pub fn set_player_id(&mut self, player_id: GameObjectId) {
        self.player_id = Some(player_id);
    }
}

impl GameObject for Finish {
    fn fixed_update(
        &mut self,
        ctx: &microengine::context::Context,
        scene: &microengine::Scene,
    ) -> microengine::error::GameResult {
        let player = scene
            .gameobject_by_id::<Player>(self.player_id.as_ref().unwrap())
            .unwrap();
        let to_player = player.get_position() - self.position;
        if to_player.magnitude() <= player.get_radius() {
            let finished_time = ctx.time.get_timestamp() as f32 - self.start_time;
            println!(
                "Congratulations you have finished the labirynth in: {} seconds!",
                finished_time
            );
            ctx.window.close();
        }
        Ok(())
    }
    fn start(
        &mut self,
        ctx: &microengine::context::Context,
        _scene: &microengine::Scene,
    ) -> microengine::error::GameResult {
        self.start_time = ctx.time.get_timestamp() as f32;
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
