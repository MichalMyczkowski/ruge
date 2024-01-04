use microengine::{ 
    components::camera::{
        Camera,
        ProjectionType,
    },
    components::transform::Space,
    GameObject, KeyCode,
};
use std::f32::consts::PI;

pub enum CameraType {
    FirstPerson,
    SideView,
}

impl From<usize> for CameraType {
    fn from(value: usize) -> Self {
        match value {
            0 => CameraType::FirstPerson,
            _ => CameraType::SideView,
        }
    }
}



pub trait CameraObject: GameObject {
    fn get_camera(&self) -> &Camera;
    fn get_camera_mut(&mut self) -> &mut Camera;
}

pub struct FirstPersonCam {
    camera: Camera,
    sensitivity: f32,
}

impl FirstPersonCam {
    pub fn new(width: f32, height: f32, sensitivity: f32) -> Self {
        Self {
            camera: Camera::new(
                        ProjectionType::Perspective { fov: 45.0 },
                        0.01,
                        1000.0, 
                        width, 
                        height
                    ),
            sensitivity,
        }
    }
}

impl GameObject for FirstPersonCam {
    fn update(&mut self, ctx: &microengine::context::Context, _scene: &microengine::Scene) -> microengine::error::GameResult {
        self.camera.update_projection(ctx.window.width() as f32, ctx.window.height() as f32);
        let offset_x = ctx.input.mouse.position_delta.0 * -1.0;
        let offset_y = ctx.input.mouse.position_delta.1 * -1.0;
        self.camera.transform.rotate(glm::Vec3::y(), offset_x * self.sensitivity * ctx.time.delta_time() as f32, Space::Local);
        self.camera.transform.rotate(glm::Vec3::x(), offset_y * self.sensitivity * ctx.time.delta_time() as f32, Space::Local);
        Ok(()) 
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CameraObject for FirstPersonCam {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }
    fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

pub struct SideViewCam {
    camera: Camera,
    speed: f32,
}

impl SideViewCam {
    pub fn new(width: f32, height: f32) -> Self {
        let mut camera = Camera::new(
            ProjectionType::Ortographic,
            0.01,
            1000.0, 
            width, 
            height
        );
        camera.transform.position.x = -4.0;
        camera.transform.rotate_euler(glm::Vec3::new(0.0, -PI/2.0, 0.0), Space::World);
        Self {
            camera,
            speed: 2.0,
        }
    }
}

impl GameObject for SideViewCam {
    fn update(&mut self, ctx: &microengine::context::Context, _scene: &microengine::Scene) -> microengine::error::GameResult {
        // TODO! FIX THIS SHIT XD
        //self.camera.update_projection(ctx.window.width() as f32, ctx.window.height() as f32);
        self.camera.update_projection(4.0, 3.0);

        if ctx.input.kb.get_key(KeyCode::KeyLeft) || ctx.input.kb.get_key(KeyCode::KeyA) {
            self.camera.transform.position.z -= self.speed * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyRight) || ctx.input.kb.get_key(KeyCode::KeyD) {
            self.camera.transform.position.z += self.speed * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyUp) || ctx.input.kb.get_key(KeyCode::KeyW) {
            self.camera.transform.position.y += self.speed * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyDown) || ctx.input.kb.get_key(KeyCode::KeyS) {
            self.camera.transform.position.y -= self.speed * ctx.time.delta_time() as f32;
        }
        Ok(()) 
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CameraObject for SideViewCam {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }
    fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

