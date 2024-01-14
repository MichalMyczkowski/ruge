pub mod mesh;
use mesh::BubbleMesh;
use super::light::{LightColor, PointLight, LightType, LightObject};
use super::light_proxy::LightProxy;
use super::player::Player;

use std::cell::RefCell;
use std::iter;

use rand::{
    rngs::StdRng,
    RngCore
};
use crate::utils::string_to_rng;
use microengine::prelude::*;
use microengine::components::transform::{*, self};

pub enum CollisionType {
    None,
    Good(u32),
    Bad(f32),
}


pub struct Bubble {
    transform: Transform,
    start_time: f32,
    max_time: f32,
    speed: f32,
    is_dead: RefCell<bool>,
    /// x position in color texture
    color_idx: f32,
    distance_to_camera: f32,
}

impl Bubble {
    pub fn new(mut transform: Transform, time: f32, max_time: f32, color_idx: f32) -> Self {
        *transform.scale_mut() *= 0.05;
        Self {
            transform,
            start_time: time,
            max_time,
            speed: 1.0,
            color_idx,
            is_dead: RefCell::new(false),
            distance_to_camera: 0.0,
        }
    }

    pub fn set_distance_from_camera(&mut self, camera_pos: &glm::Vec3) {
        self.distance_to_camera = glm::distance(camera_pos, self.transform.position());
    }

    pub fn is_dead(&self) -> bool {
        *self.is_dead.borrow()
    }
    pub fn update(&mut self, ctx: &Context) {
        if self.max_time < ctx.time.get_timestamp() as f32 - self.start_time {
            *self.is_dead.borrow_mut() = true;
        } else {
            self.transform.position_mut().y += self.speed * ctx.time.delta_time() as f32;
            self.transform.position_mut().z += 2.0 * self.speed * ctx.time.delta_time() as f32;
            self.transform.scale_mut().x += 0.15 * ctx.time.delta_time() as f32;
            self.transform.scale_mut().y += 0.15 * ctx.time.delta_time() as f32;
            self.transform.scale_mut().z += 0.15 * ctx.time.delta_time() as f32;
        }
    }
    pub fn collide(
        &self,
        aa: &glm::Vec3,
        bb: &glm::Vec3,
        ) -> bool {
       
        let inv_model = glm::inverse(&self.transform.calculate_local_to_world_matrix());
        let aa = glm::vec4_to_vec3(&(inv_model * glm::Vec4::new(aa.x, aa.y, aa.z, 1.0)));
        let bb = glm::vec4_to_vec3(&(inv_model * glm::Vec4::new(bb.x, bb.y, bb.z, 1.0)));
        let position = glm::Vec3::new(0.0, 0.0, 0.0);
        let radius = self.transform.scale().x;

        let closest_point = glm::Vec3::new(
            f32::max(aa.x, f32::min(position.x, bb.x)),
            f32::max(aa.y, f32::min(position.y, bb.y)),
            f32::max(aa.z, f32::min(position.z, bb.z)),
            );
        let sq_distance =
            (closest_point.x - position.x).powi(2) +
            (closest_point.y - position.y).powi(2) +
            (closest_point.z - position.z).powi(2);
        if sq_distance <= radius.powi(2) {
            *self.is_dead.borrow_mut() = true;
            true
        } else {
            false
        }
    }
}


pub struct Bubbles {
    bubbles: Vec<Option<Bubble>>,
    transparent: bool,
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
    // good bubbles data
    good_bubbles: Vec<Bubble>,
    good_bubbles_lights: Vec<GameObjectId>,
    good_mesh: BubbleMesh,
    spawn_good_frequency: f32,
    last_good_spawn_time: f32,
    damage: f32,
}

impl Bubbles {
    pub fn new(max_bubbles: usize, spawn_frequency: f32, damage: f32, seed: &str) -> Self {
        Self {
            bubbles: iter::repeat_with(|| None).take(max_bubbles).collect(),
            bubble_count: 0,
            transparent: true,
            max_bubbles,
            bubble_lifetime: 10.0,
            last_spawn_time: 0.0,
            spawn_frequency,
            rng: string_to_rng(seed.into()),
            spawn_area: (glm::Vec3::new(-20.0, -7.75, -15.0), glm::Vec3::new(20.0, -6.25, -95.0)),
            mesh: BubbleMesh::new(0.4),
            player_id: None,
            //
            good_bubbles: Vec::new(),
            good_bubbles_lights: Vec::new(),
            good_mesh: BubbleMesh::new(0.2),
            spawn_good_frequency: 1.2,
            last_good_spawn_time: 0.0,
            damage: 0.1,

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

    fn spawn_light(&mut self, position: glm::Vec3, scene: &Scene) {
        // create light object
        //let clr = glm::Vec3::new(0.89, 0.47, 0.705);
        let clr = glm::Vec3::new(0.0, 1.0, 0.0);
        let color = LightColor::new(0.5 * clr, 1.5 * clr, 2.0 * clr);
        let point_light = PointLight::new(position, color, 1.0, 0.35, 0.44);
        let point_light = LightType::Point(point_light);
        let id = scene.add_gameobject(LightObject::new(point_light), 0).unwrap();
        self.good_bubbles_lights.push(id);
    }

    fn spawn_good_bubbles(&mut self, ctx: &Context, scene: &Scene, count: usize) {
        self.last_good_spawn_time = ctx.time.get_timestamp() as f32;
        if self.good_bubbles.len() < 16 {
           for _ in 0..count { 
                let mut t = Transform::default();
                t.rotate_euler(glm::Vec3::new(0.2, 0.1, 0.0), transform::Space::Local);
                *t.position_mut() = Self::random_position(&self.spawn_area, &mut self.rng);
                self.spawn_light(t.position().clone(), scene);
                let b = Bubble::new(t, ctx.time.get_timestamp() as f32, self.bubble_lifetime, 1.0);
                self.good_bubbles.push(b);
            }
        }
    }

    fn update_bubbles(&mut self, ctx: &Context) {
        self.bubbles.iter_mut().for_each(|b| {
            if let Some(bubble) = b {
                bubble.update(ctx);
                if bubble.is_dead() {
                    self.bubble_count -= 1;
                    *b = None;
                }
            }
        });
    }

    fn update_good_bubbles(&mut self, ctx: &Context, scene: &Scene) {
        let mut bubbles = std::mem::take(&mut self.good_bubbles);
        bubbles.iter_mut().for_each(|b| b.update(ctx));
        let lights = std::mem::take(&mut self.good_bubbles_lights);
        (self.good_bubbles, self.good_bubbles_lights) = bubbles
                .into_iter()
                .zip(lights.into_iter())
                .filter(|(bubble, light)| {
                    if let Some(l) = scene.gameobject_by_id::<LightObject>(&light) {
                        // update bubbles coresponding light
                        if let LightType::Point(ref mut p) = *l.light.borrow_mut() {
                            p.position = glm::vec3_to_vec4(bubble.transform.position());
                        }
                        if bubble.is_dead() {
                               l.kill(); 
                               false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                }).unzip();
    }

    pub fn check_collisions(
        &self,
        body_aa: &glm::Vec3,
        body_bb: &glm::Vec3,
        tail_aa: &glm::Vec3,
        tail_bb: &glm::Vec3,
        ) -> CollisionType {

        let points = self.good_bubbles.iter().map(|b| {
            if b.collide(body_aa, body_bb) ||
                b.collide(tail_aa, tail_bb) {
                1
            } else {
                0
            }
        }).sum();

        if points > 0 {
            return CollisionType::Good(points);
        }
        
        let empty = self.max_bubbles - self.bubble_count;
        let damage = self.bubbles.iter().rev().skip(empty).take(5).map(|b| {
            if let Some(ref b) = b {
                if b.collide(body_aa, body_bb) ||
                    b.collide(body_aa, body_bb) {
                    self.damage
                } else {
                    0.0
                }
            } else {
                0.0
            }
        }).sum();
        if damage > 0.0 {
            return CollisionType::Bad(damage);
        }

        CollisionType::None
    }

}

impl GameObject for Bubbles {
    fn name(&self) -> &str {
        "bubbles"
    }

    fn start(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        self.last_spawn_time = ctx.time.get_timestamp() as f32; 
        self.player_id = scene.get_gameobject_id("player");
        Ok(())
    }

    fn update(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        let time = ctx.time.get_timestamp() as f32;
        let to_spawn = (time - self.last_spawn_time) * self.spawn_frequency;
        if to_spawn >= 1.0 {
            self.spawn_bubbles(ctx, to_spawn as usize);
        }
        let to_spawn = (time - self.last_good_spawn_time) * self.spawn_good_frequency;
        if to_spawn >= 1.0 {
            self.spawn_good_bubbles(ctx, scene, to_spawn as usize);
        }

        self.update_bubbles(ctx);
        self.update_good_bubbles(ctx, scene);
        
        // input
        if ctx.input.kb.get_key_down(KeyCode::KeyT) {
            self.transparent = if self.transparent { false } else { true };
        }
        
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
        let models = self.bubbles.iter_mut().take(self.bubble_count).flat_map(|b| {
            (b.as_mut().unwrap().transform.local_to_world()).iter().map(|&x| x).collect::<Vec<f32>>()
        }).collect::<Vec<f32>>();
        let colors = self.bubbles.iter().take(self.bubble_count).map(|b| b.as_ref().unwrap().color_idx).collect::<Vec<f32>>();
        self.mesh.draw(
            &models,
            &colors,
            &projection,
            camera.transform.position(),
            ctx.time.get_timestamp() as f32,
            self.bubble_count,
            self.transparent,
            false,
        );
        
        let models = self.good_bubbles.iter_mut().flat_map(|b| {
            b.transform.local_to_world().iter().map(|&x| x).collect::<Vec<f32>>()
        }).collect::<Vec<f32>>();
        let colors = self.good_bubbles.iter().map(|b| b.color_idx).collect::<Vec<f32>>();

        self.good_mesh.draw(
            &models,
            &colors,
            &projection,
            camera.transform.position(),
            ctx.time.get_timestamp() as f32,
            self.good_bubbles.len(),
            false,
            true,
        );

        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
