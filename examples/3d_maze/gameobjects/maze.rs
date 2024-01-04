mod mesh;
use mesh::MazeMesh;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    f32::consts::PI,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use microengine::{ 
    GameObject,
    context::Context,
    Scene,
    error::GameResult,
    GameObjectId,
    components::transform::Transform,
};
use super::player::Player;


pub struct Maze {
    size: usize,
    mesh: MazeMesh,
    transform: Vec<Transform>,
    player_id: Option<GameObjectId>,
}

impl Maze {

    fn string_to_rng(seed: String) -> StdRng {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        hasher.finish();
        StdRng::seed_from_u64(hasher.finish())
    }

    pub fn new(size: usize, seed: String) -> Self {
        let mut r = Self::string_to_rng(seed);
        // calculate transforms
        let mut transform = Vec::with_capacity(size.pow(3));
        for y in 0..size {
            for z in 0..size {
                for x in 0..size {
                    let (rot_x, rot_y, rot_z) = (
                            (r.next_u32() as f32) / (u32::MAX as f32) * PI,
                            (r.next_u32() as f32) / (u32::MAX as f32) * PI,
                            (r.next_u32() as f32) / (u32::MAX as f32) * PI,
                    );
                    let scale_factor = (r.next_u32() as f32) / (u32::MAX as f32) + 0.2;
                    transform.push(Transform::new(
                        glm::Vec3::new(x as f32, y as f32, z as f32),
                        glm::Vec3::new(rot_x, rot_y, rot_z),
                        //glm::Vec3::new(0.0, 0.0, 0.0),
                        glm::Vec3::new(scale_factor, scale_factor, scale_factor),
                        //glm::Vec3::new(1.0, 1.0, 1.0),
                    ));

                }
            }
        }
        let mesh = MazeMesh::new(0.8, size.pow(3), &mut transform, size);
        Self {
            size,
            transform,
            mesh,
            player_id: None,
        }
    }

    // TODO! get transform of tetrahedron on position x y z
    fn get_transform(&self, x: usize, y: usize, z: usize) -> Option<&Transform> {
    
        None
    }
    pub fn set_player_id(&mut self, player_id: GameObjectId) {
        self.player_id = Some(player_id);
    }

}

impl GameObject for Maze {
    fn draw(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        let player = scene.gameobject_by_id::<Player>(self.player_id.as_ref().unwrap()).unwrap();
        let projection = player.active_camera().world_to_projection_matrix();
        self.mesh.draw(projection, ctx.time.get_timestamp() as f32);
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
