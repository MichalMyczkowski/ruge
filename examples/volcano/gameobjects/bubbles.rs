pub mod mesh;
use mesh::BubbleMesh;
use super::player::Player;

use std::iter;

use rand::{
    rngs::StdRng,
    RngCore
};
use crate::utils::string_to_rng;
use microengine::prelude::*;
use microengine::components::transform::{*, self};


pub struct Bubble {
    transform: Transform,
    start_time: f32,
    max_time: f32,
    speed: f32,
    is_dead: bool,
    /// x position in color texture
    color_idx: f32,
    distance_to_camera: f32,
}

impl Bubble {
    pub fn new(transform: Transform, time: f32, max_time: f32, color_idx: f32) -> Self {
        Self {
            transform,
            start_time: time,
            max_time,
            speed: 1.0,
            color_idx,
            is_dead: false,
            distance_to_camera: 0.0,
        }
    }

    pub fn set_distance_from_camera(&mut self, camera_pos: &glm::Vec3) {
        self.distance_to_camera = glm::distance(camera_pos, self.transform.position());
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }
    pub fn update(&mut self, ctx: &Context) {
        if self.max_time < ctx.time.get_timestamp() as f32 - self.start_time {
            self.is_dead = true;
        } else {
            self.transform.position_mut().y += self.speed * ctx.time.delta_time() as f32;
        }
    }
}


pub struct Bubbles {
    bubbles: Vec<Option<Bubble>>,
    bubble_count: usize,
    max_bubbles: usize,
    spawn_area: (glm::Vec3, glm::Vec3),
    bubble_lifetime: f32,
    /// Spawn frequency should be in Hz
    spawn_frequency: f32,
    last_spawn_time: f32,
    rng: StdRng,
    mesh: BubbleMesh,
    player_id: Option<GameObjectId>,
}

impl Bubbles {
    pub fn new(max_bubbles: usize, spawn_frequency: f32, seed: &str) -> Self {
        Self {
            bubbles: iter::repeat_with(|| None).take(max_bubbles).collect(),
            bubble_count: 0,
            max_bubbles,
            bubble_lifetime: 8.0,
            last_spawn_time: 0.0,
            spawn_frequency,
            rng: string_to_rng(seed.into()),
            spawn_area: (glm::Vec3::new(-20.0, -0.75, -15.0), glm::Vec3::new(20.0, -0.25, -95.0)),
            mesh: BubbleMesh::new(0.4),
            player_id: None,
        }
    }

    fn random_position(spawn_area: &(glm::Vec3, glm::Vec3), rng: &mut StdRng) -> glm::Vec3 {
        let mut x = rng.next_u32() as f32 / u32::MAX as f32;
        x = x * (spawn_area.1.x - spawn_area.0.x) + spawn_area.0.x;
        let mut y = rng.next_u32() as f32 / u32::MAX as f32;
        y = y * (spawn_area.1.y - spawn_area.0.y) + spawn_area.0.y;
        let mut z = rng.next_u32() as f32 / u32::MAX as f32;
        z = z * (spawn_area.1.z - spawn_area.0.z) + spawn_area.0.z;
        glm::Vec3::new(x, y, z)

    }

    fn spawn_bubbles(&mut self, ctx: &Context, count: usize) {
        self.last_spawn_time = ctx.time.get_timestamp() as f32;
        let capacity = self.max_bubbles - self.bubble_count;
        let to_add = if capacity > count { count } else { capacity };
        self.bubbles.iter_mut().rev().take(to_add).for_each(|b| {
            let mut t = Transform::default();
            t.rotate_euler(glm::Vec3::new(0.2, 0.1, 0.0), transform::Space::Local);
            *t.position_mut() = Self::random_position(&self.spawn_area, &mut self.rng);
            let clr = self.rng.next_u32() as f32 / u32::MAX as f32;
            self.bubble_count += 1;
            b.replace(Bubble::new(t, ctx.time.get_timestamp() as f32, self.bubble_lifetime, clr));
        });

    }
}

impl GameObject for Bubbles {
    fn start(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        self.last_spawn_time = ctx.time.get_timestamp() as f32; 
        let player_id = scene.get_gameobject_id("player").unwrap();
        self.player_id = Some(player_id);
        Ok(())
    }

    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        let time = ctx.time.get_timestamp() as f32;
        let to_spawn = (time - self.last_spawn_time) * self.spawn_frequency;
        if to_spawn >= 1.0 {
            self.spawn_bubbles(ctx, to_spawn as usize);
        }

        self.bubbles.iter_mut().for_each(|b| {
            if let Some(bubble) = b {
                bubble.update(ctx);
                if bubble.is_dead() {
                    self.bubble_count -= 1;
                    // TODO: do something fancy on death
                    *b = None;
                }
            }
        });
        
        Ok(())
    }


    fn draw(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        if self.bubble_count == 0 {
            return Ok(());
        }
        let player = scene.gameobject_by_id::<Player>(self.player_id.as_ref().unwrap()).unwrap();
        let camera = player.active_camera();
        self.bubbles.iter_mut().for_each(|b| {
            if let Some(b) = b {
                b.set_distance_from_camera(camera.transform.position());
            }
        });
        self.bubbles.sort_unstable_by(|a, b| {
            match (a, b) {
                (Some(a), Some(b)) => {
                    b.distance_to_camera.partial_cmp(&a.distance_to_camera).unwrap()
                },
                (Some(_), None) => {
                    std::cmp::Ordering::Less
                },
                (None, Some(_)) => {
                    std::cmp::Ordering::Greater
                },
                (None, None) => {
                    std::cmp::Ordering::Equal
                }
            }
        });
       
        let projection = camera.world_to_projection_matrix();
        let mvps = self.bubbles.iter_mut().take(self.bubble_count).flat_map(|b| {
            (projection * b.as_mut().unwrap().transform.local_to_world()).iter().map(|&x| x).collect::<Vec<f32>>()
        }).collect::<Vec<f32>>();
        let colors = self.bubbles.iter().take(self.bubble_count).map(|b| b.as_ref().unwrap().color_idx).collect::<Vec<f32>>();
        self.mesh.draw(&mvps, &colors, ctx.time.delta_time() as f32, self.bubble_count);


        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
