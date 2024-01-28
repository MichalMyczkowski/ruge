pub mod cube_face;
pub mod chunk;

use std::f32::consts::{ FRAC_PI_2, PI};
use microengine::{components::transform::{Space, Transform}, prelude::*};
use crate::config::debug;

use super::player::Player;
use cube_face::CubeFace;

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
        let radius = 1.0;
        Self {
            cube_map: [
                CubeFace::new(radius * glm::Vec3::new(-0.5, 0.5, 0.5), glm::Vec3::x(), -glm::Vec3::y(), radius , glm::Vec3::new(1.0, 0.0, 0.0)),
                CubeFace::new(radius * glm::Vec3::new(0.5, 0.5, -0.5), -glm::Vec3::y(), glm::Vec3::z(), radius , glm::Vec3::new(0.0, 1.0, 0.0)),
                CubeFace::new(radius * glm::Vec3::new(-0.5, 0.5, -0.5), glm::Vec3::x(), glm::Vec3::z(), radius , glm::Vec3::new(0.0, 0.0, 1.0)),
                CubeFace::new(radius * glm::Vec3::new(-0.5, -0.5, -0.5), glm::Vec3::x(), glm::Vec3::y(), radius, glm::Vec3::new(1.0, 1.0, 0.0)),
                CubeFace::new(radius * glm::Vec3::new(-0.5, -0.5, -0.5), glm::Vec3::y(), glm::Vec3::z(), radius, glm::Vec3::new(0.0, 1.0, 1.0)),
                CubeFace::new(radius * glm::Vec3::new(-0.5, -0.5, 0.5), glm::Vec3::x(), -glm::Vec3::z(), radius, glm::Vec3::new(1.0, 0.0, 1.0)),
            ],
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
        let camera_pos = player.get_position();
        let frustum = player.get_frustum();
        let mvp = player.active_camera().world_to_projection_matrix();

        let mix = Self::ease_in_out(self.lerp);
        for i in 0..6 {
            self.cube_map[i].draw(&mvp, camera_pos, frustum, mix);
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
