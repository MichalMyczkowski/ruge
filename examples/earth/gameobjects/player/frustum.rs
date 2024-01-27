use gl_utils::{CompiledProgram, primitives};
use microengine::components::camera::{Camera, ProjectionType};

pub struct Plane (glm::Vec4);

impl Default for Plane {
    fn default() -> Self {
        Self (glm::Vec4::new(0.0, 0.0, 0.0, 1.0))
    }
}

pub struct Frustum {
    pub top: Plane,
    pub bottom: Plane,
    pub left: Plane,
    pub right: Plane,
    pub near: Plane,
    pub far: Plane,
}

impl Frustum {
    pub fn calculate(&mut self, camera: &Camera) {
        let vp = camera.world_to_projection_matrix();
        let r1 = glm::Vec4::new(vp.m11, vp.m12, vp.m13, vp.m14);
        let r2 = glm::Vec4::new(vp.m21, vp.m22, vp.m23, vp.m24);
        let r3 = glm::Vec4::new(vp.m31, vp.m32, vp.m33, vp.m34);
        let r4 = glm::Vec4::new(vp.m41, vp.m42, vp.m43, vp.m44);

        self.left.0 = r1 + r4;
        self.right.0 = r1 - r4;
        self.bottom.0 = r2 + r4;
        self.top.0 = r2 - r4;
        self.near.0 = r3 + r4;
        self.far.0 = r3 - r4;
    }

    pub fn in_frustum(&self, point: &glm::Vec3) -> bool {
        let point = glm::Vec4::new(point.x, point.y, point.z, 1.0);
        glm::dot(&self.left.0, &point) >= 0.0 &&
        glm::dot(&self.right.0, &point) <= 0.0 &&
        glm::dot(&self.top.0, &point) <= 0.0 &&
        glm::dot(&self.bottom.0, &point) >= 0.0 &&
        glm::dot(&self.near.0, &point) >= 0.0 &&
        glm::dot(&self.far.0, &point) <= 0.0



        //self.left.is_in_front(point)?;
        //self.right.is_in_front(point)?;
        //self.top.is_in_front(point)?;
        //self.bottom.is_in_front(point)?;
        //self.near.is_in_front(point)?;
        //self.far.is_in_front(point)?;
        //Ok(Inside)
    }

}

impl Default for Frustum {
    fn default() -> Self {
        Self {
            top: Default::default(),
            bottom: Default::default(),
            left: Default::default(),
            right: Default::default(),
            near: Default::default(),
            far: Default::default(),
        }
    }
}

pub struct FrustumMesh {
    program: CompiledProgram,
}

const VERT_SHADER_PATH: &str = "./examples/earth/gameobjects/player/shaders/frustum.vs";
const FRAG_SHADER_PATH: &str = "./examples/earth/gameobjects/player/shaders/frustum.fs";

impl FrustumMesh {

    pub fn new() -> Self {
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
        };
        t.set_buffers();
        t
    }

    fn set_buffers(&self) {
        let frustum = [
            glm::Vec3::new(-1.0, -1.0, 1.0),
            glm::Vec3::new(1.0, -1.0, 1.0),

            glm::Vec3::new(1.0, -1.0, 1.0),
            glm::Vec3::new(1.0, 1.0, 1.0),

            glm::Vec3::new(1.0, 1.0, 1.0),
            glm::Vec3::new(-1.0, 1.0, 1.0),

            glm::Vec3::new(-1.0, 1.0, 1.0),
            glm::Vec3::new(-1.0, -1.0, 1.0),

            glm::Vec3::new(-1.0, -1.0, -1.0),
            glm::Vec3::new(1.0, -1.0, -1.0),

            glm::Vec3::new(1.0, -1.0, -1.0),
            glm::Vec3::new(1.0, 1.0, -1.0),

            glm::Vec3::new(1.0, 1.0, -1.0),
            glm::Vec3::new(-1.0, 1.0, -1.0),

            glm::Vec3::new(-1.0, 1.0, -1.0),
            glm::Vec3::new(-1.0, -1.0, -1.0),

            glm::Vec3::new(-1.0, -1.0, 1.0),
            glm::Vec3::new(-1.0, -1.0, -1.0),

            glm::Vec3::new(1.0, -1.0, 1.0),
            glm::Vec3::new(1.0, -1.0, -1.0),

            glm::Vec3::new(1.0, 1.0, 1.0),
            glm::Vec3::new(1.0, 1.0, -1.0),

            glm::Vec3::new(-1.0, 1.0, 1.0),
            glm::Vec3::new(-1.0, 1.0, -1.0),
        ];
        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (3 * frustum.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                frustum.as_ptr() as *const gl::types::GLvoid,
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

    pub fn draw(&self, mvp: glm::Mat4, inv_projection: glm::Mat4) {
        self.program.bind_program();
        self.program.bind_vao();
        unsafe {
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("mvp"),
                1,
                gl::FALSE, 
                mvp.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("inv_projection"),
                1,
                gl::FALSE, 
                inv_projection.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::DrawArrays(
                gl::LINES,
                0,
                24,
            );
        }
    }
}
