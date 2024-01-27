use microengine::components::transform::Transform;

use crate::gameobjects::player::frustum::Frustum;
use super::chunk::ChunkMesh;

struct QDrawing<'a> {
    LOD: usize,
    MAX_LOD: usize,
    mix: f32,
    projection: &'a glm::Mat4,
    camera_pos: &'a glm::Vec3,
    // texture_proxy: 
}

pub struct CubeFace {
    chunk: ChunkMesh,
    start_pos: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    radius: f32,
}

impl CubeFace {
    pub fn new(face_transform: &mut Transform, start_pos: glm::Vec3, right: glm::Vec3, up: glm::Vec3, radius: f32) -> Self {
        Self {
            chunk: ChunkMesh::new(face_transform, radius),
            start_pos,
            right,
            up,
            radius,
        }
    }

    fn q_draw(&mut self, aux: &QDrawing) {
       //  
    }

    pub fn draw(&mut self, projection: &glm::Mat4, camera_pos: &glm::Vec3, frustum: &Frustum, mix: f32) {
        self.chunk.bind_shader();
        //// TODO: calculate max LOD
        //let aux = QDrawing {
        //    LOD: 0,
        //    MAX_LOD: 11,
        //    mix,
        //    projection,
        //    camera_pos,
        //};
        if frustum.in_frustum(&self.start_pos) ||
            frustum.in_frustum(&(self.start_pos + self.right)) ||
            frustum.in_frustum(&(self.start_pos + self.up)) ||
            frustum.in_frustum(&(self.start_pos + self.right + self.up)) {
            self.chunk.draw(projection, &glm::Vec3::zeros(), &glm::Vec3::new(1.0, 1.0, 1.0), mix);
        }
    }
}
