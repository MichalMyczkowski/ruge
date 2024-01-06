mod mesh;
use mesh::MazeMesh;
use nalgebra_glm::inverse;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    f32::consts::PI, iter,
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
use crate::utils::closestPointTriangle;


pub struct Maze {
    size: usize,
    mesh: MazeMesh,
    transform: Vec<Transform>,
    player_id: Option<GameObjectId>,
    // for collision detection
    close_cells: Vec<(isize, isize, isize)>,
    triangles: Vec<Vec<glm::Vec3>>,
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
        // cells in 3d space for faster collision calculation
        let mut close_cells = Vec::with_capacity(3usize.pow(3));
        for x in 0..3isize {
            for y in 0..3isize {
                for z in 0..3isize {
                    close_cells.push((x - 1, y - 1, z - 1));
                }
            }
        }
        let height = 0.8;
        let mesh = MazeMesh::new(height, size.pow(3), &mut transform, size);
        Self {
            size,
            transform,
            mesh,
            player_id: None,
            close_cells,
            triangles: MazeMesh::tetrahedron_triangles(height)
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    // TODO! get transform of tetrahedron on position x y z
    fn get_transform(&self, x: usize, y: usize, z: usize) -> Option<&Transform> {
        let size = self.size;
        if x >= size || y >= size || z >= size {
            None
        } else {
            let idx = (y * size.pow(2)) + (z * size) + x;
            self.transform.get(idx)
        }
    }

    /// Returns minimal distance from given point to closest obstacle in maze.
    /// Checks only distance to obstacles that are in 'cells' around point.
    /// omits tetrahedron 0
    pub fn distance_to_obstacle(&self, point: &glm::Vec3) -> Option<f32> {
        let (p_x, p_y, p_z) = (point.x as isize, point.y as isize, point.z as isize);
        let obstacles: Vec<&Transform> = self.close_cells.iter().filter(|(x, y, z)| {
            if p_x + x < 0 || p_y + y < 0 || p_z + z < 0 {
                false
            } else {
                true
            }
        }).filter_map(|(x, y, z)| {
            if *x == 0 && *y == 0 && *z == 0 {
                None
            } else {
                self.get_transform((*x + p_x) as usize, (*y + p_y) as usize, (*z + p_z) as usize)
            }
        }).collect();
       
        let point4 = glm::Vec4::new(point.x, point.y, point.z, 1.0);
        let mut min_dist = None;
        obstacles.iter().for_each(|t| {
            let model = t.calculate_local_to_world_matrix();
            let inv = glm::inverse(&model);
            let p = inv * point4;
            let p = glm::vec4_to_vec3(&p);
            for tri in self.triangles.iter() {
                let closest = closestPointTriangle(&p, &tri[0], &tri[1], &tri[2]);
                let closest = model * glm::Vec4::new(closest.x, closest.y, closest.z, 1.0);
                let distance = glm::distance(&closest, &point4);
                if min_dist.is_none() || min_dist.unwrap() > distance {
                    min_dist = Some(distance);
                }
                
            }
        });
        
        min_dist

    }

}

impl GameObject for Maze {
    fn start(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        let player_id = scene.get_gameobject_id("player").unwrap();
        self.player_id = Some(player_id);
        Ok(())
    }
    fn draw(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        let player = scene.gameobject_by_id::<Player>(self.player_id.as_ref().unwrap()).unwrap();
        let projection = player.active_camera().world_to_projection_matrix();
        self.mesh.draw(projection, ctx.time.get_timestamp() as f32);
        Ok(())
    }
    fn name(&self) -> &str {
        "maze"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
