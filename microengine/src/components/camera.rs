/// Basic camera component implementation
///
use super::transform::Transform;

#[derive(Clone, Copy)]
pub enum ProjectionType {
    Ortographic,
    Perspective { fov: f32 },
}

pub struct Camera {
    pub transform: Transform,
    pub projection: Projection,
    window_dim: (f32, f32),
}

impl Camera {
    /// Creates a new camera
    pub fn new(
        projection: ProjectionType,
        near_clip_plane: f32,
        far_clip_plane: f32,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            transform: Transform::default(),
            projection: Projection::new(projection, near_clip_plane, far_clip_plane),
            window_dim: (width, height),
        }
    }

    /// Returns world to projection transformation matrix
    pub fn world_to_projection_matrix(&self) -> glm::Mat4 {
        let inverse_angle = glm::quat_angle(&self.transform.rotation) * -1.0;
        let inverse_rotation =
            glm::quat_angle_axis(inverse_angle, &glm::quat_axis(&self.transform.rotation));
        let view = glm::translation(&(self.transform.position() * -1.0));
        let view = glm::quat_to_mat4(&inverse_rotation) * view;
        let projection = self.projection.projection_matrix();
        projection * view
    }

    /// Should be called each time window dimensions change
    pub fn update_projection(&mut self, width: f32, height: f32) {
        if self.window_dim != (width, height) {
            self.window_dim = (width, height);
            self.projection.update(width, height);
        }
    }

    pub fn front(&self) -> glm::Vec3 {
        self.transform.vector_to_world(&(-glm::Vec3::z()))
    }

    pub fn up(&self) -> glm::Vec3 {
        self.transform.vector_to_world(&(glm::Vec3::y()))
    }

    pub fn right(&self) -> glm::Vec3 {
        self.transform.vector_to_world(&(glm::Vec3::x()))
    }

    pub fn aspect(&self) -> f32 {
        self.window_dim.0 / self.window_dim.1
    }
}

pub struct Projection {
    pub p_type: ProjectionType,
    pub z_near: f32,
    pub z_far: f32,
    matrix: glm::Mat4,
}

impl Projection {
    pub fn new(p_type: ProjectionType, z_near: f32, z_far: f32) -> Self {
        Self {
            p_type,
            z_near,
            z_far,
            matrix: match p_type {
                ProjectionType::Ortographic => glm::ortho(0.0, 800.0, 0.0, 600.0, z_near, z_far),
                ProjectionType::Perspective { fov } => glm::perspective(
                    1.33,
                    glm::radians(&glm::Vec1::new(fov)).x as f32,
                    z_near,
                    z_far,
                ),
            },
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        // TODO! update fov?
        self.matrix = match self.p_type {
            ProjectionType::Ortographic => {
                glm::ortho(0.0, width, 0.0, height, self.z_near, self.z_far)
            }
            ProjectionType::Perspective { fov } => glm::perspective(
                width / height,
                glm::radians(&glm::Vec1::new(fov)).x as f32,
                self.z_near,
                self.z_far,
            ),
        }
    }

    pub fn projection_matrix(&self) -> &glm::Mat4 {
        &self.matrix
    }
}
