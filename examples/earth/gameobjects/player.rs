mod cameras;
mod mesh;
pub mod frustum;

use crate::config::debug;
use mesh::PlayerMesh;
use frustum::{ Frustum, FrustumMesh };
use cameras::*;
use microengine::components::transform::Space;
use microengine::prelude::*;
use microengine::components::{
    camera::Camera,
    transform::Transform
};

#[derive(Clone, Copy)]
enum Mode {
    ArcBall,
    Debug,
}
impl From<usize> for Mode {
    fn from(value: usize) -> Self {
        match value {
            0 => Mode::ArcBall,
            _ => Mode::Debug,
        }
    }
}

impl From<Mode> for usize {
    fn from(value: Mode) -> Self {
        match value {
            Mode::ArcBall => 0,
            Mode::Debug => 1,
        }
    }
}

pub struct Player {
    mesh: PlayerMesh,
    frustum_mesh: FrustumMesh,
    frustum: Frustum,
    cameras: Vec<Box<dyn CameraObject>>,
    camera_index: usize,
    mode: Mode,
}

impl Player {
    pub fn new() -> Self {
        Self {
            mesh: PlayerMesh::new(0.07),
            frustum_mesh: FrustumMesh::new(),
            frustum: Default::default(),
            cameras: Vec::with_capacity(3),
            camera_index: 0,
            mode: Mode::ArcBall,
        }
    }

    pub fn active_camera(&self) -> &Camera {
        self.cameras[self.mode as usize].get_camera()
    }

    pub fn get_position(&self) -> &glm::Vec3 {
        self.cameras[Mode::ArcBall as usize].get_camera().transform.position()
    }

    pub fn get_frustum(&self) -> &Frustum {
        &self.frustum
    }

}

impl GameObject for Player {
    fn start(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        ctx.input.mouse.set_cursor_visibility(false);
        self.cameras.push(
            Box::new(
                ArcBallCam::new(
                    ctx.window.width() as f32,
                    ctx.window.height() as f32,
                    0.1,
                    3.0,
                )
            ),
        );
        self.cameras.push(
            Box::new(
                FirstPersonCam::new(
                    ctx.window.width() as f32, 
                    ctx.window.height() as f32, 
                    0.1,
                )
            ),
        );
        Ok(())
    }

    fn update(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        if ctx.input.kb.get_key_down(KeyCode::KeyTab) && debug() {
            self.camera_index = match self.mode {
                Mode::ArcBall => Mode::Debug as usize,
                Mode::Debug => Mode::ArcBall as usize,
            };
            self.mode = self.camera_index.into();
            println!("switched to: {}", self.camera_index);
        }
        // update camera
        for idx in 0..self.cameras.len() {
            let is_active = if idx == self.camera_index { true } else { false };
            self.cameras[idx].update(ctx, is_active);
        }
        self.frustum.calculate(self.cameras[Mode::ArcBall as usize].get_camera());
        Ok(())
    }

    fn draw(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        // TODO: draw self in debug mode
        if self.camera_index == Mode::Debug as usize {
            self.mesh.draw(
                self.cameras[Mode::Debug as usize].get_camera_mut().world_to_projection_matrix() * 
                self.cameras[Mode::ArcBall as usize].get_camera_mut().transform.local_to_world(), ctx.time.delta_time() as f32);
            self.frustum_mesh.draw(
                self.cameras[Mode::Debug as usize].get_camera_mut().world_to_projection_matrix(), 
                glm::inverse(&self.cameras[Mode::ArcBall as usize].get_camera_mut().world_to_projection_matrix()));
            Ok(())
        } else {
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
