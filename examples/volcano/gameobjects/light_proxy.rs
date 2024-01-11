use std::cell::{RefCell, RefMut};

use gl_utils::CompiledProgram;
use microengine::{prelude::*, error::GameError};
use super::light::*;
use crate::config::debug;

pub const MAX_LIGHT_COUNT: usize = 16;
pub const LIGHT_BUFFER_BINDING_POINT: u32 = 1;

const DIR_STRUCT_SIZE: usize = 4 + 12;
const POINT_STRUCT_SIZE: usize = 4 + 12 + 4;
const SPOT_STRUCT_SIZE: usize = 4 + 4 + 12 + 4;
const DIR_STRUCT_OFFSET: usize = 0;
const POINT_STRUCT_OFFSET: usize = DIR_STRUCT_OFFSET + DIR_STRUCT_SIZE * 16;
const SPOT_STRUCT_OFFSET: usize = POINT_STRUCT_OFFSET + POINT_STRUCT_SIZE * 16;
const DIR_COUNT_OFFSET: usize = SPOT_STRUCT_OFFSET + SPOT_STRUCT_SIZE * 16;
const POINT_COUNT_OFFSET: usize = DIR_COUNT_OFFSET + 1;
const SPOT_COUNT_OFFSET: usize = POINT_COUNT_OFFSET + 1;

pub struct LightProxy {
    directional_lights: Vec<f32>,
    point_lights: Vec<f32>,
    spot_lights: Vec<f32>,

    registered_directional_lights: RefCell<Vec<GameObjectId>>,
    registered_point_lights: RefCell<Vec<GameObjectId>>,
    registered_spot_lights: RefCell<Vec<GameObjectId>>,

    ubo: u32,
}

impl LightProxy {
    pub fn new() -> Self {
        let mut ubo: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, LIGHT_BUFFER_BINDING_POINT, ubo); 
            gl::BufferData(
                gl::UNIFORM_BUFFER, 
                ((
                        16 * DIR_STRUCT_SIZE +
                        16 * POINT_STRUCT_SIZE +
                        16 * SPOT_STRUCT_SIZE +
                        4
                ) * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                std::ptr::null(), 
                gl::DYNAMIC_DRAW
            );
        }
        Self {
            directional_lights: Vec::with_capacity(0),
            point_lights: Vec::with_capacity(0),
            spot_lights: Vec::with_capacity(0),
            registered_directional_lights: RefCell::new(Vec::with_capacity(MAX_LIGHT_COUNT)),
            registered_point_lights: RefCell::new(Vec::with_capacity(MAX_LIGHT_COUNT)),
            registered_spot_lights: RefCell::new(Vec::with_capacity(MAX_LIGHT_COUNT)),
            ubo,

        }
    }
    
    fn register_light(mut vec: RefMut<Vec<GameObjectId>>, light_id: GameObjectId, light_type: &str) -> GameResult {
        println!("registering: {light_type}");
        if vec.len() == MAX_LIGHT_COUNT {
            return Err(GameError::Error(String::from("Trying to register more than supported lights of type: ") + light_type));
        }
        vec.push(light_id);
        Ok(())
    }

    fn unregister_light(mut vec: RefMut<Vec<GameObjectId>>, light_id: GameObjectId, light_type: &str) -> GameResult {
        println!("unregistering: {light_type}");
        let idx = vec.iter().position(|x| *x == light_id);
        match idx {
            Some(idx) => {vec.remove(idx);},
            None => return Err(GameError::Error(String::from("Trying to unregister unregistered light of type: ") + light_type)),
        }
        
        Ok(())
    }

    pub fn register_directional_light(&self, light_id: GameObjectId) -> GameResult {
        let vec = self.registered_directional_lights.borrow_mut();
        Self::register_light(vec, light_id, "directional light")
    }
    pub fn unregister_directional_light(&self, light_id: GameObjectId) -> GameResult {
        let vec = self.registered_directional_lights.borrow_mut();
        Self::unregister_light(vec, light_id, "directional light")
    }
    pub fn register_point_light(&self, light_id: GameObjectId) -> GameResult {
        let vec = self.registered_point_lights.borrow_mut();
        Self::register_light(vec, light_id, "point light")
    }
    pub fn unregister_point_light(&self, light_id: GameObjectId) -> GameResult {
        let vec = self.registered_point_lights.borrow_mut();
        Self::unregister_light(vec, light_id, "point light")
    }
    pub fn register_spot_light(&self, light_id: GameObjectId) -> GameResult {
        let vec = self.registered_spot_lights.borrow_mut();
        Self::register_light(vec, light_id, "spot light")
    }
    pub fn unregister_spot_light(&self, light_id: GameObjectId) -> GameResult {
        let vec = self.registered_spot_lights.borrow_mut();
        Self::unregister_light(vec, light_id, "spot light")
    }

    pub fn set_uniforms(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
            // dir lights
            if self.directional_lights.len() > 0 {
                gl::BufferSubData(
                    gl::UNIFORM_BUFFER,
                    (DIR_STRUCT_OFFSET) as gl::types::GLsizeiptr,
                    (DIR_STRUCT_SIZE * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    self.directional_lights.as_ptr() as *const gl::types::GLvoid,
                );
                let dir_len = self.registered_directional_lights.borrow().len() as i32;
                let dir_len = vec![dir_len];
                gl::BufferSubData(
                    gl::UNIFORM_BUFFER,
                    (DIR_COUNT_OFFSET * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    (std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                    dir_len.as_ptr() as *const gl::types::GLvoid,
                );
            }
            // spot lights
            if self.spot_lights.len() > 0 {
                gl::BufferSubData(
                    gl::UNIFORM_BUFFER,
                    (SPOT_STRUCT_OFFSET * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    (self.spot_lights.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    self.spot_lights.as_ptr() as *const gl::types::GLvoid,
                );
                let spot_len = self.registered_spot_lights.borrow().len() as i32;
                let spot_len = vec![spot_len];
                gl::BufferSubData(
                    gl::UNIFORM_BUFFER,
                    (SPOT_COUNT_OFFSET * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    (std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                    spot_len.as_ptr() as *const gl::types::GLvoid,
                );
            }
        }
    }

    fn update_vectors(&mut self, scene: &Scene) {
        self.directional_lights = self.registered_directional_lights.borrow()
            .iter()
            .filter_map(|id| {
                scene.gameobject_by_id::<LightObject>(id)
            })
            .flat_map(|light| light.as_vec())
            .collect::<Vec<f32>>();

        self.point_lights = self.registered_point_lights.borrow()
            .iter()
            .filter_map(|id| {
                scene.gameobject_by_id::<LightObject>(id)
            })
            .flat_map(|light| light.as_vec())
            .collect::<Vec<f32>>();

        self.spot_lights = self.registered_spot_lights.borrow()
            .iter()
            .filter_map(|id| {
                scene.gameobject_by_id::<LightObject>(id)
            })
            .flat_map(|light| light.as_vec())
            .collect::<Vec<f32>>();
    }

}

impl GameObject for LightProxy {
    fn start(&mut self, _ctx: &Context, _scene: &Scene) -> GameResult {
        Ok(())
    }

    fn update(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        self.update_vectors(scene);
        self.set_uniforms();
        Ok(())
    }

    fn name(&self) -> &str {
        "light"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
