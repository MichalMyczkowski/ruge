pub mod cube_face;
pub mod chunk;

use std::f32::consts::{ FRAC_PI_2, PI};
use microengine::{components::transform::{Space, Transform}, prelude::*};
use crate::config::debug;

use super::player::Player;
use cube_face::CubeFace;
use chunk::ChunkMesh;

pub enum EarthShape {
    Sphere,
    Cube,
}


pub struct Earth {
    cube_map: [CubeFace; 6],
    radius: f32,
    shape: EarthShape,
    lerp: f32,
    transition_speed: f32,
    player_id: Option<GameObjectId>,
}

impl Earth {
    pub fn new() -> Self {
        // transforms for cube walls
        let mut t0 = Transform::default();
        *t0.scale_mut() = glm::Vec3::new(0.5, 1.0, 0.5);
        let mut t1 = t0.clone();
        let mut t2 = t0.clone();
        let mut t3 = t0.clone();
        let mut t4 = t0.clone();
        let mut t5 = t0.clone();
        *t0.position_mut() += glm::Vec3::new(0.0, 0.0, 0.5);
        t0.rotate(glm::Vec3::x(), FRAC_PI_2, Space::World);
        *t1.position_mut() += glm::Vec3::new(0.5, 0.0, 0.0);
        t1.rotate(glm::Vec3::z(), FRAC_PI_2, Space::World);
        *t2.position_mut() += glm::Vec3::new(0.0, 0.5, 0.0);

        *t3.position_mut() -= glm::Vec3::new(0.0, 0.0, 0.5);
        t3.rotate(glm::Vec3::x(), FRAC_PI_2 * -1.0, Space::World);
        *t4.position_mut() -= glm::Vec3::new(0.5, 0.0, 0.0);
        t4.rotate(glm::Vec3::z(), FRAC_PI_2 * -1.0, Space::World);
        *t5.position_mut() -= glm::Vec3::new(0.0, 0.5, 0.0);
        t5.rotate(glm::Vec3::x(), PI, Space::World);

        let radius = 1.0;
        let cube_map = [
            CubeFace::new(&mut t0, radius),
            CubeFace::new(&mut t1, radius),
            CubeFace::new(&mut t2, radius),
            CubeFace::new(&mut t3, radius),
            CubeFace::new(&mut t4, radius),
            CubeFace::new(&mut t5, radius),
        ];
        Self {
            cube_map,
            shape: EarthShape::Sphere,
            radius,
            transition_speed: 1.0,
            lerp: 0.0,
            player_id: None,
        }
    }

    fn ease_direction(&self) -> f32 {
        match self.shape {
            EarthShape::Cube => 1.0,
            EarthShape::Sphere => -1.0,
        }
    }

    fn ease_in_out(x: f32) -> f32 {
        if x < 0.5 {
            4.0 * x.powi(3)
        } else {
            1.0 - ( (-2.0 * x + 2.0).powi(3) / 2.0 )
        }
    }
}

impl GameObject for Earth {
    fn start(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        let player_id = scene.get_gameobject_id("player").unwrap();
        self.player_id = Some(player_id);
        Ok(())
    }

    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        if ctx.input.kb.get_key_down(KeyCode::KeyT) && debug() {
            self.shape = match self.shape {
                EarthShape::Cube => EarthShape::Sphere,
                EarthShape::Sphere => EarthShape::Cube,
            };
        }
        self.lerp += self.ease_direction() * self.transition_speed * ctx.time.delta_time() as f32;
        if self.lerp < 0.0 {
            self.lerp = 0.0;
        }
        if self.lerp > 1.0 {
            self.lerp = 1.0;
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        let player = scene.gameobject_by_id::<Player>(self.player_id.as_ref().unwrap()).unwrap();
        let mvp = player.active_camera().world_to_projection_matrix();

        let mix = Self::ease_in_out(self.lerp);
        for i in 0..6 {
            self.cube_map[i].draw(&mvp, mix);
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
