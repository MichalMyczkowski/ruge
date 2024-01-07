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



pub struct Player {
    transform: Transform,
    mesh: PlayerMesh,
    radius: f32,
    cameras: Vec<Box<dyn CameraObject>>,
    camera_index: usize,
    reached_goal: bool,
    // moving
    speed: glm::Vec3,
    acceleration: f32,
    friction: f32,
    max_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        let radius = 0.2;
        let mut transform = Transform::default();
        *transform.scale_mut() = glm::Vec3::new(radius, radius, radius);
        Self {
            transform,
            mesh: PlayerMesh::new(radius),
            radius,
            cameras: Vec::with_capacity(3),
            camera_index: 0,
            speed: glm::Vec3::zeros(),
            reached_goal: false,
            acceleration: 0.1,
            friction: 0.95,
            max_speed: 3.0,
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
        Ok(())
    }

    fn fixed_update(&mut self, _ctx: &Context, _scene: &Scene) -> GameResult {
        *self.transform.position_mut() = self.transform.position() + self.speed;
        Ok(())
    }

    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
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
        Ok(())
    }

    fn draw(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        if let CameraType::FirstPerson = CameraType::from(self.camera_index) {
            Ok(())
        } else {
            self.mesh.draw(
                self.active_camera().world_to_projection_matrix() * self.transform.local_to_world(),
                ctx.time.get_timestamp() as f32
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
