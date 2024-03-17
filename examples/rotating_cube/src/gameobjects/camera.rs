use microengine::prelude::*;
use microengine::components::{transform::Transform, camera::{Camera, ProjectionType}};

pub const CAMERA_NAME: &str = "camera";

pub fn add_camera(scene: &mut Scene) {
    let camera = CameraObject::new();
    _ = scene.add_gameobject(camera, 1).unwrap()
}

pub struct CameraObject {
    pub camera: Camera,
}

impl CameraObject {
    pub fn new() -> CameraObject {
        let mut t = Transform::default();
        t.position_mut().z = 5.0;
        let mut camera = Camera::new(
            ProjectionType::Perspective { fov: 45.0 },
            0.01,
            1000.0,
            1.0f32,
            1.0f32,
        );
        camera.transform = t;
        CameraObject {
            camera
        }
    }
}

impl GameObject for CameraObject {

    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        self.camera.update_projection(ctx.window.width() as f32, ctx.window.height() as f32);
        Ok(())
    }

    fn name(&self) -> &str {
        CAMERA_NAME
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
