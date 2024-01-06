mod mesh;

use mesh::AxisMesh;
use microengine::{ 
    KeyCode,
    GameObject,
    context::Context,
    Scene,
    error::GameResult,
    GameObjectId,
    components::transform::Transform,
};

use crate::config::debug;
use super::player::Player;


pub struct Axis {
    transform: Transform,
    mesh: AxisMesh,
    player_id: Option<GameObjectId>,
}

impl Axis {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            mesh: AxisMesh::new(),
            player_id: None,
        }
    }

}

impl GameObject for Axis {
    fn start(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        let player_id = scene.get_gameobject_id("player").unwrap();
        self.player_id = Some(player_id);
        Ok(())
    }
    
    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        if ctx.input.kb.get_key_down(KeyCode::KeyEscape) {
            ctx.window.close();
        }
        if ctx.input.kb.get_key_down(KeyCode::KeyF) {
            if ctx.window.is_fullscreen() {
                ctx.window.set_fullscreen(false);
            } else {
                ctx.window.set_fullscreen(true);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        if debug() {
            let player = scene.gameobject_by_id::<Player>(self.player_id.as_ref().unwrap()).unwrap();
            let mvp = player.active_camera().world_to_projection_matrix() * self.transform.local_to_world();
            self.mesh.draw(mvp, ctx.time.get_timestamp() as f32);
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
