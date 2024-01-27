use microengine::components::transform::Transform;

use super::chunk::ChunkMesh;


pub struct CubeFace {
    chunk: ChunkMesh,
    radius: f32,
}

impl CubeFace {
    pub fn new(face_transform: &mut Transform, radius: f32) -> Self {
        Self {
            chunk: ChunkMesh::new(face_transform, radius),
            radius,
        }
    }

    pub fn draw(&mut self, projection: &glm::Mat4, mix: f32) {
        self.chunk.bind_shader();
        self.chunk.draw(projection, &glm::Vec3::zeros(), &glm::Vec3::new(1.0, 1.0, 1.0), mix);
    }
}
