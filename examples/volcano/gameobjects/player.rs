mod cameras;
mod mesh;

use crate::config::debug;
use mesh::PlayerMesh;
use cameras::*;
use microengine::prelude::*;
use microengine::components::{
    camera::Camera,
    transform::Transform
};

use super::bubbles::{Bubbles, CollisionType};
use super::light::{LightColor, SpotLight, LightType, LightObject};

pub struct Player {
    pub transform: Transform,
    tail_transform: Transform,
    blade1_transform: Transform,
    blade2_transform: Transform,
    mesh: PlayerMesh,
    cameras: Vec<Box<dyn CameraObject>>,
    camera_index: usize,
    reached_goal: bool,
    // moving
    speed: glm::Vec3,
    acceleration: f32,
    friction: f32,
    max_speed: f32,
    // other gameobjects
    light_id: Option<GameObjectId>,
    bubbles_id: Option<GameObjectId>,
    // damage and points
    points: u32,
    damage: f32,

    
}

impl Player {
    pub fn new() -> Self {
        let mut tail_transform = Transform::new(
            glm::Vec3::new(0.0, 0.25, 1.0),
            glm::Vec3::new(0.0, 0.0, 0.0),
            glm::Vec3::new(0.15, 0.15, 1.7),
            );
        tail_transform.rotate(glm::Vec3::x(), -0.3, microengine::components::transform::Space::Local);
        let mut transform = Transform::default();

        let blade1_transform = Transform::new(
            glm::Vec3::new(0.0, 0.5, 0.0),
            glm::Vec3::new(0.0, std::f32::consts::PI/2.0, 0.0),
            glm::Vec3::new(0.25, 0.25, 2.5),
            );
        let blade2_transform = Transform::new(
            glm::Vec3::new(0.0, 0.5, 0.0),
            glm::Vec3::new(0.0, std::f32::consts::PI, 0.0),
            glm::Vec3::new(0.25, 0.25, 2.5),
            );

        *transform.scale_mut() = glm::Vec3::new(0.3, 0.3, 0.3);
        Self {
            transform,
            tail_transform,
            blade1_transform,
            blade2_transform,
            mesh: PlayerMesh::new(),
            cameras: Vec::with_capacity(3),
            camera_index: 0,
            speed: glm::Vec3::zeros(),
            reached_goal: false,
            acceleration: 0.8,
            friction: 0.95,
            max_speed: 7.0,
            light_id: None,
            bubbles_id: None,
            damage: 0.0,
            points: 0,
        }
    }

    pub fn active_camera(&self) -> &Camera {
        self.cameras[self.camera_index].get_camera()
    }

    pub fn reached_goal(&self) -> bool {
        self.reached_goal
    }

    fn next_camera(&mut self) {
        self.camera_index = (self.camera_index + 1) % self.cameras.len();
    }

    fn move_player(&mut self, ctx: &microengine::context::Context) {
        let cam = self.cameras[self.camera_index].get_camera_mut();

        let v_front = cam.transform.vector_to_world(&(glm::Vec3::z() * -1.0));
        let v_right = cam.transform.vector_to_world(&glm::Vec3::x());
        let v_back = v_front * -1.0;
        let v_left = v_right * -1.0;

        let mut front = glm::Vec3::zeros();
        let mut right = glm::Vec3::zeros();
        let mut back = glm::Vec3::zeros();
        let mut left = glm::Vec3::zeros();
        
        // input
        if ctx.input.kb.get_key(KeyCode::KeyW) {
            front = v_front * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyS) {
            back = v_back * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyD) {
            right = v_right * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyA) {
            left = v_left * self.acceleration * ctx.time.delta_time() as f32;
        }
        self.speed += front + back + right + left;
        if self.speed.magnitude() > self.max_speed {
            self.speed = (self.speed / self.speed.magnitude()) * self.max_speed;
        }
        self.speed *= self.friction;
        if self.speed.magnitude() <= 0.0005 {
            self.speed = glm::Vec3::zeros();
        }
    }

    fn update_light(&mut self, light: &LightObject) {
        let mut l = light.light.borrow_mut();
        if let LightType::Spot(ref mut l) = *l {
            l.position = glm::vec3_to_vec4(self.transform.position());
            l.direction = glm::vec3_to_vec4(&self.active_camera().transform.vector_to_world(&(glm::Vec3::z() * -1.0)));
        }
    }
}

impl GameObject for Player {
    fn start(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        if !debug() {
            ctx.input.mouse.set_cursor_visibility(false);
        }
        self.cameras.push(
            Box::new(
                FirstPersonCam::new(
                    ctx.window.width() as f32, 
                    ctx.window.height() as f32, 
                    0.1,
                )
            ),
        );
        self.cameras.push(
            Box::new(
                SideViewCam::new(
                    ctx.window.width() as f32,
                    ctx.window.height() as f32,
                    3.0,
                )
            ),
        );
        self.cameras.push(
            Box::new(
                ThirdPersonCam::new(
                    ctx.window.width() as f32, 
                    ctx.window.height() as f32, 
                    0.01,
                    3.0,
                    &self.transform,
                )
            ),
        );
        // create light object
        let color = LightColor::new(glm::Vec3::new(0.0, 0.0, 0.0), glm::Vec3::new(0.6, 0.6, 0.6), glm::Vec3::new(0.8, 0.8, 0.8));
        let spot_light = SpotLight::new(*self.transform.position(), -1.0 * glm::Vec3::z(), color, 0.99, 0.97);
        let spot_light = LightType::Spot(spot_light);
        self.light_id = Some(scene.add_gameobject(LightObject::new(spot_light), 0).unwrap()); 
        let bubbles_id = scene.get_gameobject_id("bubbles").unwrap();
        self.bubbles_id= Some(bubbles_id);
        Ok(())
    }

    fn fixed_update(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        *self.transform.position_mut() = self.transform.position() + self.speed;
        if let Some(ref id) = self.bubbles_id {
            let bubbles = scene.gameobject_by_id::<Bubbles>(id).unwrap();
            
            let body_aa = 
                glm::vec4_to_vec3(
                    &(self.transform.local_to_world() *
                    glm::Vec4::new(0.0, 0.0, 0.0, 1.0))
                );
            let body_bb = 
                glm::vec4_to_vec3(
                    &(self.transform.local_to_world() *
                    glm::Vec4::new(0.0, 0.0, -1.0, 1.0))
                );

            let tail_aa = 
                glm::vec4_to_vec3(
                    &(self.transform.local_to_world() * self.tail_transform.local_to_world() *
                    glm::Vec4::new(0.0, 0.0, 0.0, 1.0))
                );
            let tail_bb = 
                glm::vec4_to_vec3(
                    &(self.transform.local_to_world() * self.tail_transform.local_to_world() *
                    glm::Vec4::new(0.0, 0.0, -1.0, 1.0))
                );


            let collision = bubbles.check_collisions(&body_aa, &body_bb, &tail_aa, &tail_bb);
            match collision {
                CollisionType::Good(points) => { 
                    self.points += points;
                    println!("got points: {points}");
                },
                CollisionType::Bad(damage) => {
                    self.damage += damage;
                    println!("got damage: {damage}");
                },
                CollisionType::None => (),
            }
        }
        Ok(())
    }

    fn update(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        self.blade1_transform.rotate(glm::Vec3::y(), 7.0 * ctx.time.delta_time() as f32, microengine::components::transform::Space::Local);
        self.blade2_transform.rotate(glm::Vec3::y(), 7.0 * ctx.time.delta_time() as f32, microengine::components::transform::Space::Local);

        if ctx.input.kb.get_key_down(KeyCode::KeyTab) {
            self.next_camera();
        }
        // update camera
        for idx in 0..self.cameras.len() {
            let is_active = if idx == self.camera_index { true } else { false };
            self.cameras[idx].update(ctx, &self.transform, is_active);
        }

        match CameraType::from(self.camera_index) {
            CameraType::SideView => {
            },
            CameraType::FirstPerson => {
                self.move_player(ctx);
            },
            CameraType::ThirdPerson => {
                self.move_player(ctx);
            }
        }
        // update light
        if let Some(ref id) = self.light_id {
            if let Some(ref light) = scene.gameobject_by_id::<LightObject>(id) {
                self.update_light(light);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        if let CameraType::FirstPerson = CameraType::from(self.camera_index) {
            Ok(())
        } else {
            let body = self.transform.local_to_world();
            let tail = self.tail_transform.local_to_world();
            let blade1 = self.blade1_transform.local_to_world();
            let blade2 = self.blade2_transform.local_to_world();
            self.mesh.draw(
                self.active_camera().transform.position(),
                &self.active_camera().world_to_projection_matrix(),
                &body,
                &tail,
                &blade1,
                &blade2,
                ctx.time.get_timestamp() as f32,
                self.damage,
            );
            Ok(()) 
        }
    }

    fn name(&self) -> &str {
        "player"
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
