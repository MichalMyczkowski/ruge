use microengine::{ 
    context::Context,
    components::{camera::{
        Camera,
        ProjectionType,
    }, transform},
    components::transform::Space,
    GameObject, KeyCode,
};
use std::f32::consts::PI;

pub enum CameraType {
    FirstPerson,
    SideView,
    ThirdPerson,
}

impl From<usize> for CameraType {
    fn from(value: usize) -> Self {
        match value {
            0 => CameraType::FirstPerson,
            1 => CameraType::SideView,
            _ => CameraType::ThirdPerson,
        }
    }
}



pub trait CameraObject {
    fn get_camera(&self) -> &Camera;
    fn get_camera_mut(&mut self) -> &mut Camera;
    fn update(&mut self, ctx: &Context, player_transform: &transform::Transform, is_active: bool);
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

impl CameraObject for FirstPersonCam {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }

    fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    fn update(&mut self, ctx: &Context, player_transform: &transform::Transform, is_active: bool) {
        self.camera.update_projection(ctx.window.width() as f32, ctx.window.height() as f32);
        if is_active {
            let offset_x = ctx.input.mouse.position_delta.0 * -1.0;
            let offset_y = ctx.input.mouse.position_delta.1 * -1.0;
            self.camera.transform.rotate(glm::Vec3::y(), offset_x * self.sensitivity * ctx.time.delta_time() as f32, Space::World);
            self.camera.transform.rotate(glm::Vec3::x(), offset_y * self.sensitivity * ctx.time.delta_time() as f32, Space::Local);
            *self.camera.transform.position_mut() = *player_transform.position();
        }
    }
}

pub struct SideViewCam {
    camera: Camera,
    speed: f32,
    fixed_height: f32,
}

impl SideViewCam {
    pub fn new(width: f32, height: f32, fixed_height: f32) -> Self {
        let mut camera = Camera::new(
            ProjectionType::Ortographic,
            0.01,
            1000.0, 
            width, 
            height
        );
        camera.transform.position_mut().x = -4.0;
        camera.transform.rotate_euler(glm::Vec3::new(0.0, -PI/2.0, 0.0), Space::World);
        Self {
            camera,
            speed: 2.0,
            fixed_height,
        }
    }
}

impl GameObject for SideViewCam {
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
    fn update(&mut self, ctx: &Context, player_transform: &transform::Transform, is_active: bool) {
        let width = (ctx.window.width() as f32 / ctx.window.height() as f32) * self.fixed_height;
        self.camera.update_projection(width, self.fixed_height);
        if is_active {
            if ctx.input.kb.get_key(KeyCode::KeyLeft) || ctx.input.kb.get_key(KeyCode::KeyA) {
                self.camera.transform.position_mut().z -= self.speed * ctx.time.delta_time() as f32;
            }
            if ctx.input.kb.get_key(KeyCode::KeyRight) || ctx.input.kb.get_key(KeyCode::KeyD) {
                self.camera.transform.position_mut().z += self.speed * ctx.time.delta_time() as f32;
            }
            if ctx.input.kb.get_key(KeyCode::KeyUp) || ctx.input.kb.get_key(KeyCode::KeyW) {
                self.camera.transform.position_mut().y += self.speed * ctx.time.delta_time() as f32;
            }
            if ctx.input.kb.get_key(KeyCode::KeyDown) || ctx.input.kb.get_key(KeyCode::KeyS) {
                self.camera.transform.position_mut().y -= self.speed * ctx.time.delta_time() as f32;
            }
        } else {
            self.camera.transform.position_mut().z = player_transform.position().z - width / 2.0;
            self.camera.transform.position_mut().y = player_transform.position().y - self.fixed_height / 2.0;
        }
    }
}

// -- -- -- -- --

pub struct ThirdPersonCam {
    camera: Camera,
    radius: f32,
    sensitivity: f32,
    from_player: glm::Vec3,
}

impl ThirdPersonCam {
    pub fn new(width: f32, height: f32, sensitivity: f32, radius: f32, player_transform: &transform::Transform) -> Self {
        let mut camera = Camera::new(
            ProjectionType::Perspective { fov: 45.0 },
            0.6,
            1000.0, 
            width, 
            height
        );
        let from_player = glm::Vec3::z() * radius;
        *camera.transform.position_mut() = *player_transform.position() + from_player;
        Self {
            camera,
            sensitivity,
            radius,
            from_player,
        }
    }
}


impl CameraObject for ThirdPersonCam {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }
    fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    fn update(&mut self, ctx: &Context, player_transform: &transform::Transform, is_active: bool) {
        // TODO: USE PLAYER_TRANSFORM
        self.camera.update_projection(ctx.window.width() as f32, ctx.window.height() as f32);
        let offset_x = ctx.input.mouse.position_delta.0 * -1.0;
        let offset_y = ctx.input.mouse.position_delta.1 * 1.0;
        
        let up = self.camera.transform.vector_to_world(&glm::Vec3::y());
        let right = self.camera.transform.vector_to_world(&glm::Vec3::x());

        if offset_x != 0.0 || offset_y != 0.0 {
            let new_pos = self.from_player + self.sensitivity * (offset_y * up + offset_x * right);
            let rot_axis = glm::cross(
                &self.from_player,
                &new_pos
            );

            let angle = glm::angle(
                &self.from_player,
                &new_pos
            );

            self.camera.transform.rotate(rot_axis, angle, transform::Space::World);
            
            self.from_player = new_pos.normalize() * self.radius;
        }
        // Apply new camera position
        *self.camera.transform.position_mut() = *player_transform.position() + self.from_player;

    }
}
