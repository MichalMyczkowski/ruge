mod mesh;
use mesh::SunMesh;
use microengine::{prelude::*, components::transform::Transform};

use super::{light::{LightType, DirectionalLight, LightColor, LightObject}, player::Player};

pub struct Sun {
    transform: Transform,
    distance: f32,
    mesh: SunMesh,
    player_id: Option<GameObjectId>,
    light_id: Option<GameObjectId>,
}

impl Sun {
    pub fn new(direction: glm::Vec3, distance: f32) -> Self {
        let position = glm::normalize(&direction) * -distance;
        let mut t = Transform::default();
        *t.position_mut() = position;
        Self {
            transform: t,
            distance,
            mesh: SunMesh::new(20.0),
            player_id: None,
            light_id: None,
        }
    }
}


impl GameObject for Sun {
    fn start(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        // get player id
        let player_id = scene.get_gameobject_id("player").unwrap();
        self.player_id = Some(player_id);
        // create light object
        let color = LightColor::new(glm::Vec3::new(0.1, 0.1, 0.1), glm::Vec3::new(0.4, 0.4, 0.4), glm::Vec3::new(0.9, 0.9, 0.9));
        let dir_light = DirectionalLight::new(self.transform.position() - glm::Vec3::zeros(), color);
        let dir_light = LightType::Directional(dir_light);
        self.light_id = Some(scene.add_gameobject(LightObject::new(dir_light), 0).unwrap()); 

        self.transform.position_mut().y = 1.0;
        Ok(())
    }

    fn update(&mut self, ctx: &Context, scene: &Scene) -> GameResult {
        self.transform.position_mut().x = (ctx.time.get_timestamp() as f32 / 20.0).sin();
        self.transform.position_mut().z = (ctx.time.get_timestamp() as f32 / 20.0).cos();
        self.transform.position_mut().y = 0.7;

        // get lightobject 
        if let Some(ref id) = self.light_id {
            if let Some(ref light) = scene.gameobject_by_id::<LightObject>(id) {
                if let LightType::Directional(ref mut d) = *light.light.borrow_mut() {
                    let direction = glm::normalize(&(glm::Vec3::zeros() - *self.transform.position()));
                    d.direction = glm::vec3_to_vec4(&direction);
                }
            }
        }
        // get player
        if let Some(ref id) = self.player_id {
            let player = scene.gameobject_by_id::<Player>(id).unwrap();
            *self.transform.position_mut() = *player.transform.position() + (*self.transform.position() * self.distance);
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        if let Some(ref id) = self.player_id {
            let player = scene.gameobject_by_id::<Player>(id).unwrap();
            self.mesh.draw(player.active_camera().world_to_projection_matrix() * self.transform.local_to_world(), self.transform.position());
        }

        Ok(())
    }
        
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
