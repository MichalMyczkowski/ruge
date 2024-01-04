/// Provides Transform component which holds position, rotation and scale data
/// and some useful methods.

#[derive(Copy, Clone)]
pub enum Space {
    Local,
    World,
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Quat,
    pub scale: glm::Vec3,
    local_to_world: glm::Mat4,
    changed: bool,
}

impl Transform {
    pub fn new(position: glm::Vec3, rotation: glm::Vec3, scale: glm::Vec3) -> Self {
        let mut t = Self::default();
        t.position = position;
        t.rotate_euler(rotation, Space::World);
        t.scale = scale;
        t
    }

    /// Returns local to world transformation matrix
    pub fn local_to_world(&mut self) -> glm::Mat4 {
        if self.changed {
            let wrld = glm::translation(&self.position);
            let wrld = glm::scale(&wrld, &self.scale);
            self.local_to_world = wrld * glm::quat_to_mat4(&self.rotation);

            //let wrld = glm::scaling(&self.scale);
            //let wrld = wrld * glm::quat_to_mat4(&self.rotation);
            //self.local_to_world = glm::translate(&wrld, &self.position);
            self.changed = false;
        }
        self.local_to_world
    }

    /// Rotates object by angles.z, angles.x and angles.y in that order
    /// object is rotated either using local or world axis depending on relative_to enum
    pub fn rotate_euler(&mut self, angles: glm::Vec3, relative_to: Space) {
        self.rotate(glm::Vec3::z(), angles.z, relative_to);
        self.rotate(glm::Vec3::x(), angles.x, relative_to);
        self.rotate(glm::Vec3::y(), angles.y, relative_to);
    }

    /// Rotates object around provided axis (world axis or local) by given angle
    pub fn rotate(&mut self, axis: glm::Vec3, angle: f32, relative_to: Space) {
        let axis = axis.normalize();
        let q_rotation = glm::quat_angle_axis(angle, &axis);
        match relative_to {
            Space::Local => {
                self.rotation = glm::quat_rotate_normalized_axis(&self.rotation, angle, &axis);
            }
            Space::World => {
                self.rotation = glm::quat_rotate_normalized_axis(
                    &q_rotation,
                    glm::quat_angle(&self.rotation),
                    &glm::quat_axis(&self.rotation),
                );
            }
        }
        self.rotation = self.rotation.normalize();
    }

    // /// Transforms a vector from world space to local space
    // pub fn vector_to_local(&self, v: &glm::Vec3) -> glm::Vec3 {
    //     // TODO!?
    //     glm::quat_rotate_vec3(&self.rotation.conjugate(), v)
    // }

    /// Transforms a vector from local space to world space
    pub fn vector_to_world(&self, v: &glm::Vec3) -> glm::Vec3 {
        glm::quat_rotate_vec3(&self.rotation, v)
    }

    /// Return rotation in euler angles (pitch, yaw, roll)
    pub fn euler_angles(&self) -> glm::Vec3 {
        glm::quat_euler_angles(&self.rotation)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: glm::Vec3::new(0.0, 0.0, 0.0),
            rotation: glm::quat_angle_axis(std::f32::consts::PI * 2.0, &glm::Vec3::z()),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
            local_to_world: glm::Mat4::default(),
            changed: true,
        }
    }
}
