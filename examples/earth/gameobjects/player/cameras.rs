use microengine::{ 
    context::Context,
    components::{camera::{
        Camera,
        ProjectionType,
    }, transform::{self, Transform}},
    components::transform::Space,
    GameObject, KeyCode,
};
use std::f32::consts::PI;


pub trait CameraObject {
    fn get_camera(&self) -> &Camera;
    fn get_camera_mut(&mut self) -> &mut Camera;
    fn update(&mut self, ctx: &Context, is_active: bool);
}

pub struct FirstPersonCam {
    camera: Camera,
    sensitivity: f32,
    speed: glm::Vec3,
    acceleration: f32,
    friction: f32,
    max_speed: f32,
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
            speed: glm::Vec3::zeros(),
            acceleration: 0.8,
            friction: 0.95,
            max_speed: 7.0,
        }
    }
    fn move_camera(&mut self, ctx: &microengine::context::Context) {
        let v_front = self.camera.transform.vector_to_world(&(glm::Vec3::z() * -1.0));
        let v_right = self.camera.transform.vector_to_world(&glm::Vec3::x());
        let v_back = v_front * -1.0;
        let v_left = v_right * -1.0;

        let mut front = glm::Vec3::zeros();
        let mut right = glm::Vec3::zeros();
        let mut back = glm::Vec3::zeros();
        let mut left = glm::Vec3::zeros();
        
        // input
        if ctx.input.kb.get_key(KeyCode::KeyW) {
            front = v_front * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyS) {
            back = v_back * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyD) {
            right = v_right * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyA) {
            left = v_left * self.acceleration * ctx.time.delta_time() as f32;
        }
        self.speed += front + back + right + left;
        if self.speed.magnitude() > self.max_speed {
            self.speed = (self.speed / self.speed.magnitude()) * self.max_speed;
        }
        self.speed *= self.friction;
        if self.speed.magnitude() <= 0.0005 {
            self.speed = glm::Vec3::zeros();
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

    fn update(&mut self, ctx: &Context, is_active: bool) {
        self.camera.update_projection(ctx.window.width() as f32, ctx.window.height() as f32);
        if is_active {
            let offset_x = ctx.input.mouse.position_delta.0 * -1.0;
            let offset_y = ctx.input.mouse.position_delta.1 * -1.0;
            self.camera.transform.rotate(glm::Vec3::y(), offset_x * self.sensitivity * ctx.time.delta_time() as f32, Space::World);
            self.camera.transform.rotate(glm::Vec3::x(), offset_y * self.sensitivity * ctx.time.delta_time() as f32, Space::Local);
            self.move_camera(ctx);
            *self.camera.transform.position_mut() += self.speed;
        }
    }
}

// -- -- -- -- --

pub struct ArcBallCam {
    arc_transform: Transform,
    camera: Camera,
    radius: f32,
    sensitivity: f32,
    from_center: glm::Vec3,
    // 
    speed: glm::Vec3,
    acceleration: f32,
    friction: f32,
    max_speed: f32,
}

impl ArcBallCam {
    pub fn new(width: f32, height: f32, sensitivity: f32, radius: f32) -> Self {
        let mut camera = Camera::new(
            ProjectionType::Perspective { fov: 45.0 },
            0.6,
            1000.0, 
            width, 
            height
        );
        let from_center = glm::Vec3::z() * radius;
        let mut arc_transform = Transform::default();
        *arc_transform.position_mut() = from_center;
        Self {
            arc_transform,
            camera,
            sensitivity,
            radius,
            from_center,
            //
            speed: glm::Vec3::zeros(),
            acceleration: 0.8,
            friction: 0.95,
            max_speed: 7.0,
        }
    }

    fn ease_friction(radius: f32) -> f32 {
        assert!(radius > 0.0);
        ( radius - 1.1 ).powi(4)
    }

    fn move_camera(&mut self, ctx: &microengine::context::Context) {
        let v_front = self.camera.transform.vector_to_world(&(glm::Vec3::z() * -1.0));
        let v_right = self.camera.transform.vector_to_world(&glm::Vec3::x());
        let v_back = v_front * -1.0;
        let v_left = v_right * -1.0;

        let mut front = glm::Vec3::zeros();
        let mut right = glm::Vec3::zeros();
        let mut back = glm::Vec3::zeros();
        let mut left = glm::Vec3::zeros();
        
        // input
        if ctx.input.kb.get_key(KeyCode::KeyW) {
            front = v_front * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyS) {
            back = v_back * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyD) {
            right = v_right * self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyA) {
            left = v_left * self.acceleration * ctx.time.delta_time() as f32;
        }
        self.speed += front + back + right + left;
        if self.speed.magnitude() > self.max_speed {
            self.speed = (self.speed / self.speed.magnitude()) * self.max_speed;
        }
        self.speed *= self.friction;
        if self.speed.magnitude() <= 0.0005 {
            self.speed = glm::Vec3::zeros();
        }
    }
}


impl CameraObject for ArcBallCam {
    fn get_camera(&self) -> &Camera {
        &self.camera
    }
    fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    fn update(&mut self, ctx: &Context, is_active: bool) {
        self.camera.update_projection(ctx.window.width() as f32, ctx.window.height() as f32);
        
        // zoom in / zoom out
        if ctx.input.kb.get_key_down(KeyCode::KeyUp) {
            self.radius += 1.0 * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key_down(KeyCode::KeyDown) {
            self.radius -= 1.0 * ctx.time.delta_time() as f32;
        }
        //self.friction = Self::ease_friction(self.radius);
        if is_active {
        // camera positioning 
        self.move_camera(ctx);
        if self.speed.magnitude() > 0.0 {
            let new_pos = self.from_center + self.speed;
            let rot_axis = glm::cross(
                &self.from_center,
                &new_pos
            );
            let angle = glm::angle(
                &self.from_center,
                &new_pos
            );
            self.arc_transform.rotate(rot_axis, angle, transform::Space::World);
            self.from_center = new_pos.normalize() * self.radius;
        }
        *self.arc_transform.position_mut() = self.from_center;
        *self.camera.transform.position_mut() = self.from_center;
        // camera rotation
        let offset_x = ctx.input.mouse.position_delta.0 * -1.0;
        let offset_y = ctx.input.mouse.position_delta.1 * -1.0;
        self.camera.transform.rotate(glm::Vec3::y(), offset_x * self.sensitivity * ctx.time.delta_time() as f32, Space::Local);
        self.camera.transform.rotate(glm::Vec3::x(), offset_y * self.sensitivity * ctx.time.delta_time() as f32, Space::Local);
        }
    }
}
