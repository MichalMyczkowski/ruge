pub mod data;

use std::cell::RefCell;

pub use data::LightColor;
pub use data::DirectionalLight;
pub use data::PointLight;
pub use data::SpotLight;

use microengine::prelude::*;

use super::light_proxy::LightProxy;


pub enum LightType {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}

pub struct LightObject {
    pub light: RefCell<LightType>,
    id: Option<GameObjectId>,
    light_proxy_id: Option<GameObjectId>,
    killed: RefCell<bool>,
    dead: bool,
}

impl LightObject {
    pub fn new(light: LightType) -> Self {
        Self {
            light: RefCell::new(light),
            id: None,
            light_proxy_id: None,
            killed: RefCell::new(false),
            dead: false,
        }
    }

    pub fn kill(&self) {
        *self.killed.borrow_mut() = true;
    }

    pub fn as_vec(&self) -> Vec<f32> {
        match *self.light.borrow() {
            LightType::Directional(ref dl) => dl.as_vec(),
            LightType::Point(ref p) => p.as_vec(),
            LightType::Spot(ref sp) => sp.as_vec(),
        }
    }
}

impl GameObject for LightObject {
    fn on_add(&mut self, _ctx: &Context, scene: &Scene, id: GameObjectId) -> GameResult {
        self.id = Some(id); 
        // Register self in light_proxy
        self.light_proxy_id = scene.get_gameobject_id("light");
        if let Some(ref lp_id) = self.light_proxy_id {
            let proxy = scene.gameobject_by_id::<LightProxy>(lp_id).unwrap();
            match *self.light.borrow() {
                LightType::Directional(_) => proxy.register_directional_light(id)?,
                LightType::Point(_) => proxy.register_point_light(id)?,
                LightType::Spot(_) => proxy.register_spot_light(id)?,
            }
        } 
        Ok(())
    }

    fn update(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        // unregister self if killed
        if *self.killed.borrow() {
            self.light_proxy_id = scene.get_gameobject_id("light");
            if let Some(ref id) = self.light_proxy_id {
                let proxy = scene.gameobject_by_id::<LightProxy>(id).unwrap();
                if let Some(ref id) = self.id {
                    match *self.light.borrow() {
                        LightType::Directional(_) => proxy.unregister_directional_light(*id)?,
                        LightType::Point(_) => proxy.unregister_point_light(*id)?,
                        LightType::Spot(_) => proxy.unregister_spot_light(*id)?,
                    }
                }
            } 
            self.dead = true;
        }
        Ok(())
    }
    
    fn is_dead(&mut self) -> bool {
        self.dead
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
