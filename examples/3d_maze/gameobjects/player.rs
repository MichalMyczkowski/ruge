mod cameras;
mod mesh;

use mesh::PlayerMesh;

use cameras::{
    FirstPersonCam,
    SideViewCam,
    ThirdPersonCam,
    CameraType,
    CameraObject,
};
use microengine::{
    GameObject,
    components::{camera::Camera, transform::Transform}, KeyCode,
};



pub struct Player {
    transform: Transform,
    mesh: PlayerMesh,
    cameras: Vec<Box<dyn CameraObject>>,
    camera_index: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            mesh: PlayerMesh::new(),
            cameras: Vec::with_capacity(3),
            camera_index: 0,
        }
    }

    pub fn active_camera(&self) -> &Camera {
        self.cameras[self.camera_index].get_camera()
    }

    fn next_camera(&mut self) {
        self.camera_index = (self.camera_index + 1) % self.cameras.len();
    }

    fn move_player(&mut self, ctx: &microengine::context::Context) {
        let cam = self.cameras[self.camera_index].get_camera_mut();
        let my_position = self.transform.position_mut();

        let front = cam.transform.vector_to_world(&(glm::Vec3::z() * -1.0));
        let right = cam.transform.vector_to_world(&glm::Vec3::x());
        let speed = if ctx.input.kb.get_key(KeyCode::KeyLeftShift) {
            2.0f32
        } else {
            1.0f32
        };
        
        // input
        if ctx.input.kb.get_key(KeyCode::KeyW) {
            *my_position += front * speed * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyS) {
            *my_position += front * -speed * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyD) {
            *my_position += right * speed * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyA) {
            *my_position += right * -speed * ctx.time.delta_time() as f32;
        }

    }
}

impl GameObject for Player {
    fn start(&mut self, ctx: &microengine::context::Context, _scene: &microengine::Scene, id: microengine::GameObjectId) -> microengine::error::GameResult {
        ctx.input.mouse.set_cursor_visibility(false);
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

    fn update(&mut self, ctx: &microengine::context::Context, _scene: &microengine::Scene) -> microengine::error::GameResult {
        
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

    fn draw(&mut self, ctx: &microengine::context::Context, _scene: &microengine::Scene) -> microengine::error::GameResult {
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
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
