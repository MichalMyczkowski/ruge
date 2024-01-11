mod volcano_mesh;
mod ground_mesh;
use volcano_mesh::VolcanoMesh;
use ground_mesh::GroundMesh;
use super::player::Player;
use microengine::components::transform::Transform;
use microengine::prelude::*;

pub struct Terrain {
    pub volcano_transform: Transform,
    pub ground_transform: Transform,
    volcano_mesh: VolcanoMesh,
    ground_mesh: GroundMesh,
    player_id: Option<GameObjectId>,
}

impl Terrain {
    pub fn new() -> Self {
        let volcano_transform = Transform::new(
            glm::Vec3::new(0.0, -2.0, -100.0),
            glm::Vec3::new(0.0, 1.0, 0.0),
            glm::Vec3::new(3.0, 8.0, 3.0),
            );
        let ground_transform  = Transform::new(
            glm::Vec3::new(0.0, -16.0, -40.0),
            glm::Vec3::new(0.0, 0.0, 0.0),
            glm::Vec3::new(40.0, 1.0, 240.0),
            );
        Self {
            volcano_transform,
            volcano_mesh: VolcanoMesh::new(),
            ground_transform,
            ground_mesh: GroundMesh::new(),
            player_id: None,
        }
    }
}

impl GameObject for Terrain {
    fn start(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        let player_id = scene.get_gameobject_id("player").unwrap();
        self.player_id = Some(player_id);
        Ok(())
    }

    fn draw(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        let player = scene.gameobject_by_id::<Player>(self.player_id.as_ref().unwrap()).unwrap();
        let projection = player.active_camera().world_to_projection_matrix();
        self.volcano_mesh.draw(projection * self.volcano_transform.local_to_world(), ctx.time.get_timestamp() as f32);
        self.ground_mesh.draw(projection * self.ground_transform.local_to_world(), ctx.time.get_timestamp() as f32);
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self 
    }
}
