use std::iter;
use gl_utils::{CompiledProgram, Texture};


const VERT_SHADER_PATH: &str = "./examples/3d_maze/gameobjects/player/shaders/player.vs";
const FRAG_SHADER_PATH: &str = "./examples/3d_maze/gameobjects/player/shaders/player.fs";
const TEXTURE_PATH: &str = "./examples/3d_maze/assets/gradient.png";

// TODO! Move sphere code somewhere else
struct Ring {
    index: usize,
    segments: usize,
    points: Vec<glm::Vec3>,
}

impl Ring {
    pub fn new(radius: f32, rings: usize, segments: usize, idx: usize) -> Self {
        if idx == 0 || idx == rings + 1 {
            let radius = if idx == 0 { -1.0 * radius } else { radius };
            return Self {
                index: idx,
                segments,
                points: vec![glm::Vec3::new(0.0, radius, 0.0)],
            }
        }
        let y_spacing = (2.0 * radius) / (rings + 1) as f32;
        let y = (y_spacing * idx as f32) - radius;
        let radius = f32::sqrt(radius.powi(2) - y.powi(2));
        let mut value = 0.0f32;
        let step = std::f32::consts::PI * 2.0 / segments as f32;
        let points = iter::repeat_with(|| {
            let v = glm::Vec3::new(value.cos() * radius, y, value.sin() * radius);
            value += step;
            v
        }).take(segments).collect();

        Self {
            index: idx,
            segments,
            points,
        }
    }

    pub fn get_vert_idx(&self, idx: usize) -> usize {
        if self.index == 0 {
            0
        } else {
            let idx = idx % self.points.len();
            idx + (self.index - 1) * self.segments + 1
        }
    }
}

pub struct Sphere {
    verts: Vec<glm::Vec3>,
    indices: Vec<u32>,
}

impl Sphere {
    pub fn new(radius: f32, mut segments: usize, mut rings: usize) -> Self {
        if segments < 3 {
            segments = 3;
        }
        if rings == 0 {
            rings = 1;
        }

        let mut indices: Vec<u32> = Vec::with_capacity(3 * rings * segments * 2);
        let mut idx = 0usize;
        let rings: Vec<Ring> = iter::repeat_with(|| {
            let r = Ring::new(radius, rings, segments, idx);
            idx += 1;
            r
        }).take(rings + 2).collect();

        // each point on each ring creates two triangles
        for r_idx in 1..rings.len()-1 {
            for seg_idx in 0..segments {
                // first triangle
                indices.push(rings[r_idx].get_vert_idx(seg_idx) as u32);
                indices.push(rings[r_idx].get_vert_idx(seg_idx + 1) as u32);
                indices.push(rings[r_idx - 1].get_vert_idx(seg_idx + 1) as u32);
                // second triangle
                indices.push(rings[r_idx].get_vert_idx(seg_idx) as u32);
                indices.push(rings[r_idx + 1].get_vert_idx(seg_idx) as u32);
                indices.push(rings[r_idx].get_vert_idx(seg_idx + 1) as u32);
            }
        }

        Sphere {
            verts: rings.into_iter().flat_map(|r| r.points).collect(),
            indices,
        }
    }
}

pub struct PlayerMesh{
    program: CompiledProgram,
    texture: Texture,
    radius: f32,
    indices: usize,
}

impl PlayerMesh {

    pub fn new() -> Self {
        let radius = 0.3;
        let sphere = Sphere::new(radius, 23, 17);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            texture: Texture::from(TEXTURE_PATH),
            radius,
            indices: sphere.indices.len(),
        };
        t.set_buffers(sphere);
        t
    }

    fn set_buffers(&self, sphere: Sphere) {

        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (3 * sphere.verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                sphere.verts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (sphere.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                sphere.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // vertex data
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
        }
    }

    pub fn draw(&self, mvp: glm::Mat4, time: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        self.texture.bind_texture();
        unsafe {
            gl::Uniform1f(
                self.program.get_uniform_location("time"),
                time,
            );
            gl::Uniform1f(
                self.program.get_uniform_location("radius"),
                self.radius,
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("mvp"),
                1,
                gl::FALSE, 
                mvp.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices as i32,
                gl::UNSIGNED_INT, 
                0 as *const gl::types::GLvoid,
            );
        }
    }
}
