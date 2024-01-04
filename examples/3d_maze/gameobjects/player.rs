mod cameras;

use cameras::{
    FirstPersonCam,
    CameraObject,
};
use microengine::{
    GameObject,
    components::{camera::Camera, transform::Transform}, KeyCode,
};

use self::cameras::{SideViewCam, CameraType};


pub struct Player {
    transform: Transform,
    cameras: Vec<Box<dyn CameraObject>>,
    camera_index: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
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
}

impl GameObject for Player {
    fn start(&mut self, ctx: &microengine::context::Context, _scene: &microengine::Scene, id: microengine::GameObjectId) -> microengine::error::GameResult {
        ctx.input.mouse.set_cursor_visibility(false);
        self.transform.position.z = 3.0;
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
        Ok(())
        
    }

    fn update(&mut self, ctx: &microengine::context::Context, scene: &microengine::Scene) -> microengine::error::GameResult {
        
        if ctx.input.kb.get_key_down(KeyCode::KeyTab) {
            self.next_camera();
        }
        // update camera
        self.cameras[self.camera_index].update(ctx, scene)?;
        let cam = self.cameras[self.camera_index].get_camera_mut();

        match CameraType::from(self.camera_index) {
            CameraType::SideView => {

            },
            CameraType::FirstPerson => {
                let front = cam.transform.vector_to_world(&(glm::Vec3::z() * -1.0));
                let right = cam.transform.vector_to_world(&glm::Vec3::x());
                let speed = if ctx.input.kb.get_key(KeyCode::KeyLeftShift) {
                    2.0f32
                } else {
                    1.0f32
                };
                
                // input
                if ctx.input.kb.get_key(KeyCode::KeyW) {
                    self.transform.position += front * speed * ctx.time.delta_time() as f32;
                }
                if ctx.input.kb.get_key(KeyCode::KeyS) {
                    self.transform.position += front * -speed * ctx.time.delta_time() as f32;
                }
                if ctx.input.kb.get_key(KeyCode::KeyD) {
                    self.transform.position += right * speed * ctx.time.delta_time() as f32;
                }
                if ctx.input.kb.get_key(KeyCode::KeyA) {
                    self.transform.position += right * -speed * ctx.time.delta_time() as f32;
                }
                cam.transform.position = self.transform.position;
            },
        }


        
        Ok(())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
