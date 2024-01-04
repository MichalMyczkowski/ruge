use crate::drawable::Triangle;
use crate::drawable::TriangleType;
use crate::utils::Color;
use glm::Vec2;
use microengine::context::Context;
use microengine::error::GameResult;
use microengine::scene::Scene;
use microengine::GameObject;
use microengine::GameObjectId;
use microengine::KeyCode;

use super::Labirynth;

pub struct Player {
    drawable: Triangle,
    acceleration: f32,
    max_speed: f32,
    speed: Vec2,
    rotation: f32,
    rotate_amount: f32,
    friction: f32,
    size: f32,

    labirynth_id: Option<GameObjectId>,
    my_id: Option<GameObjectId>,
}

impl Player {
    pub fn new(position: Vec2, color: Color, size: f32) -> Self {
        Player {
            drawable: Triangle::new(
                TriangleType::Isosceles {
                    height: size,
                    base: size / 2.0,
                },
                position,
                color,
            ),
            size,
            acceleration: 0.05,
            speed: Vec2::zeros(),
            max_speed: 0.3,
            rotation: 0.0,
            rotate_amount: 2.0,
            friction: 0.95,
            labirynth_id: None,
            my_id: None,
        }
    }

    pub fn set_labirynt_id(&mut self, id: GameObjectId) {
        self.labirynth_id = Some(id);
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.drawable.position
    }

    /// approximate radius of excircle
    pub fn get_radius(&self) -> f32 {
        self.size / 3.0
    }

    pub fn read_input(&mut self, ctx: &Context) {
        // Rotate
        self.rotation = 0.0;
        if ctx.input.kb.get_key(KeyCode::KeyLeft) {
            self.rotation += self.rotate_amount * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyRight) {
            self.rotation -= self.rotate_amount * ctx.time.delta_time() as f32;
        }
        // Move
        let mut mv_forward = Vec2::zeros();
        let mut mv_backward = Vec2::zeros();
        let new_rotation = self.rotation + self.drawable.rotation;
        if ctx.input.kb.get_key(KeyCode::KeyUp) {
            mv_forward.x = new_rotation.cos();
            mv_forward.y = new_rotation.sin();
            mv_forward *= self.acceleration * ctx.time.delta_time() as f32;
        }
        if ctx.input.kb.get_key(KeyCode::KeyDown) {
            mv_backward.x = new_rotation.cos();
            mv_backward.y = new_rotation.sin();
            mv_backward *= self.acceleration * ctx.time.delta_time() as f32;
        }
        self.speed += mv_forward - mv_backward;
        if self.speed.magnitude() > self.max_speed {
            self.speed = (self.speed / self.speed.magnitude()) * self.max_speed;
        }
        self.speed *= self.friction;
        if self.speed.magnitude() <= 0.0005 {
            self.speed.x = 0.0;
            self.speed.y = 0.0;
        }
    }
}

impl GameObject for Player {
    fn start(&mut self, _ctx: &Context, _scene: &Scene, id: GameObjectId) -> GameResult {
        self.my_id = Some(id);
        Ok(())
    }

    fn fixed_update(&mut self, _ctx: &Context, scene: &Scene) -> GameResult {
        let l = scene
            .gameobject_by_id::<Labirynth>(self.labirynth_id.as_ref().unwrap())
            .unwrap();
        let verts = &self.drawable.verts;
        if l.collides(
            (&verts[0], &verts[1], &verts[2]),
            self.drawable.position + self.speed,
            self.drawable.rotation + self.rotation,
        ) {
            self.speed.x = 0.0;
            self.speed.y = 0.0;
        } else {
            self.drawable.rotation += self.rotation;
            self.drawable.position += self.speed;
        }
        Ok(())
    }

    fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        self.read_input(ctx);

        Ok(())
    }
    fn draw(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
        self.drawable.draw(ctx.window.aspect_ratio() as f32);
        Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
