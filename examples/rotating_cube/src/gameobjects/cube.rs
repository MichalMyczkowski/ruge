mod mesh;
use mesh::CubeMesh;
use microengine::prelude::*;
use microengine::components::transform::*;
use super::camera::{CAMERA_NAME, CameraObject};

pub fn add_cube(scene: &mut Scene) {
    let cube = Cube::default();
    _ = scene.add_gameobject(cube, 1).unwrap()
}

#[derive(Default)]
pub struct Cube {
    transform: Transform,
    mesh: CubeMesh,
    camera_id: Option<GameObjectId>,
}

impl GameObject for Cube {
    fn start(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        self.camera_id = scene.get_gameobject_id(CAMERA_NAME);
        Ok(())
    }

    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        self.transform.rotate(glm::Vec3::x(), ctx.time.delta_time() as f32, Space::Local);
        self.transform.rotate(glm::Vec3::z(), ctx.time.delta_time() as f32, Space::Local);
        Ok(())
    }

    fn draw(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        let camera = scene.gameobject_by_id::<CameraObject>(self.camera_id.as_ref().unwrap()).unwrap();
        self.mesh.draw(
            camera.camera.transform.position(),
            &camera.camera.world_to_projection_matrix(),
            &self.transform.local_to_world(), 
        );
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
}
