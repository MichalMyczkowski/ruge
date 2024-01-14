mod volcano_mesh;
mod ground_mesh;
use volcano_mesh::VolcanoMesh;
use ground_mesh::GroundMesh;
use super::player::Player;
use microengine::components::transform::Transform;
use microengine::prelude::*;

pub enum CollisionType {
    Ground,
    Volcano,
    Won,
    None,
}


pub struct Terrain {
    pub volcano_transform: Transform,
    pub ground_transform: Transform,
    volcano_mesh: VolcanoMesh,
    volcano_max_y: f32,
    ground_mesh: GroundMesh,
    ground_level: f32,
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
            volcano_max_y: 6.0,
            ground_transform,
            ground_mesh: GroundMesh::new(),
            ground_level: -5.0,
            player_id: None,
        }
    }

    pub fn collide(&self, position: &glm::Vec3) -> CollisionType {
        // check if is below ground level
        if position.y < self.ground_level {
            return CollisionType::Ground;
        }

        if position.y > self.volcano_max_y {
            return CollisionType::None; 
        }
        
        let center = self.volcano_transform.position();
        let outer_radius = 1.41 * self.volcano_transform.scale().x;
        let inner_radius = 1.0 * self.volcano_transform.scale().x;
        // if player is outside
        let distance = glm::distance(
            &glm::Vec2::new(position.x, position.z),
            &glm::Vec2::new(center.x, center.z)
        );

        if distance > inner_radius {
            if distance <= outer_radius {
                return CollisionType::Volcano;
            } else {
                return CollisionType::None;
            }
        }
        CollisionType::Won
    }
}

impl GameObject for Terrain {
    fn name(&self) -> &str {
        "volcano"
    }
    fn start(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        let player_id = scene.get_gameobject_id("player").unwrap();
        self.player_id = Some(player_id);
        Ok(())
    }

    fn draw(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        let player = scene.gameobject_by_id::<Player>(self.player_id.as_ref().unwrap()).unwrap();
        let projection = player.active_camera().world_to_projection_matrix();
        let cam_pos = player.active_camera().transform.position();
        self.volcano_mesh.draw(cam_pos, &projection, &self.volcano_transform.local_to_world(), ctx.time.get_timestamp() as f32);
        self.ground_mesh.draw(cam_pos, &projection, &self.ground_transform.local_to_world(), ctx.time.get_timestamp() as f32);
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self 
    }
}
