use microengine::components::transform::Transform;

use crate::gameobjects::player::frustum::Frustum;
use super::chunk::ChunkMesh;

const LOD_SPLIT_TRESHOLD: [f32; 12] = [
    2.0, 1.0, 0.5, 0.25, 0.2, 0.15, 0.10, 0.05, 0.025, 0.01, 0.005, 0.0,
];

struct QDrawing<'a> {
    lod: usize,
    max_lod: usize,
    start_pos: glm::Vec3,
    width: f32,
    projection: &'a glm::Mat4,
    camera_pos: &'a glm::Vec3,
    camera_front: &'a glm::Vec3,
    frustum: &'a Frustum,
    mix: f32,
    // texture_proxy: 
}

pub struct CubeFace {
    chunk: ChunkMesh,
    start_pos: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    initial_width: f32,
    radius: f32,
}

impl CubeFace {
    pub fn new(start_pos: glm::Vec3, right: glm::Vec3, up: glm::Vec3, radius: f32, clr: glm::Vec3) -> Self {
        Self {
            chunk: ChunkMesh::new(up, right, clr, radius),
            start_pos,
            right,
            up,
            initial_width: radius,
            radius,
        }
    }


    fn project_to_sphere(&self, point: &glm::Vec3) -> glm::Vec3 {
        glm::normalize(&point) * self.radius
    }

    fn q_draw(&mut self, aux: &mut QDrawing) {
        // compute the projection of chunk vertices to sphere:
        let sw = self.project_to_sphere(&aux.start_pos); 
        let se = self.project_to_sphere(&(aux.start_pos + self.right * aux.width)); 
        let nw = self.project_to_sphere(&(aux.start_pos + self.up * aux.width)); 
        let ne = self.project_to_sphere(&(aux.start_pos + (self.right + self.up) * aux.width)); 
        let center = self.project_to_sphere(&(aux.start_pos + (self.right + self.up) * aux.width * 0.5));

        //let norm;
        //if glm::distance(aux.camera_pos, &nw) > glm::distance(aux.camera_pos, &se) {
        //    norm = glm::triangle_normal(&se, &sw, &ne);
        //} else {
        //    norm = glm::triangle_normal(&nw, &sw, &ne);
        //}
        //if glm::dot(&norm, aux.camera_front) < 0.0 {
        //    return;
        //}
        //let dist_x = (aux.camera_pos.x - center.x).abs();
        //let dist_y = (aux.camera_pos.x - center.x).abs();
        //let dist_z = (aux.camera_pos.x - center.x).abs();
        //let l1 = f32::min(dist_x, f32::min(dist_y, dist_z));
        let l1 = glm::distance(aux.camera_pos, &center);

        if l1 <= LOD_SPLIT_TRESHOLD[aux.lod] * 1.5 && aux.lod < aux.max_lod {
            let start_pos = aux.start_pos;
            let chunk_width = aux.width * 0.5;
            let lod = aux.lod + 1;

            aux.lod = lod;
            aux.width = chunk_width;
            self.q_draw(aux);

            aux.lod = lod;
            aux.width = chunk_width;
            aux.start_pos = start_pos + self.right * chunk_width;
            self.q_draw(aux);

            aux.lod = lod;
            aux.width = chunk_width;
            aux.start_pos = nw;
            aux.start_pos = start_pos + self.up * chunk_width;
            self.q_draw(aux);

            aux.lod = lod;
            aux.width = chunk_width;
            aux.start_pos = start_pos + (self.right + self.up) * chunk_width;
            self.q_draw(aux);
        }

        if aux.frustum.in_frustum(&sw) ||
            aux.frustum.in_frustum(&se) ||
            aux.frustum.in_frustum(&nw) ||
            aux.frustum.in_frustum(&ne) {
            self.chunk.draw(aux.projection, &aux.start_pos, aux.mix, aux.width);
        }
    }

    pub fn draw(&mut self, projection: &glm::Mat4, camera_pos: &glm::Vec3, camera_front: &glm::Vec3, frustum: &Frustum, mix: f32) {
        self.chunk.bind_shader();
        //// TODO: calculate max LOD
        let mut aux = QDrawing {
            lod: 0,
            max_lod: 4,
            start_pos: self.start_pos,
            width: self.initial_width,
            projection,
            camera_pos,
            camera_front,
            frustum,
            mix,
        };
        self.q_draw(&mut aux);

    }
}
