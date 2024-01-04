use std::{
    collections::hash_map::DefaultHasher,
    f32::consts::PI,
    hash::{Hash, Hasher},
    iter,
    rc::Rc,
};

use glm::Vec2;
use microengine::GameObject;
use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::{
    drawable::LabirynthDrawable,
    utils::{collide, rotate, Color},
};

pub struct Labirynth {
    size: usize,
    rotations: Rc<Vec<Vec<f32>>>,
    positions: Rc<Vec<Vec<Vec2>>>,
    drawable: LabirynthDrawable,
}

impl Labirynth {
    fn calculate_rotations(size: usize, seed: String) -> Vec<Vec<f32>> {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        hasher.finish();
        let mut r = StdRng::seed_from_u64(hasher.finish());
        iter::repeat_with(|| {
            iter::repeat_with(|| (r.next_u32() as f32) / (u32::MAX as f32) * PI)
                .take(size)
                .collect()
        })
        .take(size)
        .collect()
    }

    fn calculate_positions(size: usize) -> Vec<Vec<Vec2>> {
        let mut row = 0;
        let cell_size = 2.0 / (size as f32);
        let offset = cell_size / 2.0 + 1.0;
        iter::repeat_with(|| {
            row += 1;
            let mut column = 0;
            iter::repeat_with(|| {
                column += 1;
                Vec2::new(
                    cell_size * column as f32 - offset,
                    cell_size * row as f32 - offset,
                )
            })
            .take(size)
            .collect::<Vec<Vec2>>()
        })
        .take(size)
        .collect()
    }

    pub fn new(size: usize, seed: String) -> Self {
        let positions = Rc::new(Self::calculate_positions(size));
        let rotations = Rc::new(Self::calculate_rotations(size, seed));
        let drawable = LabirynthDrawable::new(
            1.0 / (size as f32),
            Vec2::new(0.0, 0.0),
            Color::new(0.0, 0.0, 0.0),
            Rc::downgrade(&rotations),
            Rc::downgrade(&positions),
            size * size,
        );
        Labirynth {
            size,
            rotations,
            positions,
            drawable,
        }
    }

    fn nearby_cells(&self, position: Vec2) -> Vec<(usize, usize)> {
        let cell_size = 2.0 / (self.size as f32);
        let cell = (
            (position.x + 1.0) / cell_size,
            ((position.y) + 1.0) / cell_size,
        );
        let cell = (cell.0 as isize, cell.1 as isize);
        let dir: Vec<(isize, isize)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let len = self.size as isize;
        dir.into_iter()
            .map(|(x, y)| (cell.0 + x, cell.1 + y))
            .filter(|(x, y)| *x >= 0 && *x < len && *y >= 0 && *y < len)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| {
                if (*x == 0 && *y == 0) || (*x == self.size - 1 && *y == self.size - 1) {
                    false
                } else {
                    true
                }
            })
            .collect()
    }

    /// Checks if triangle collides with any of the labirynth "walls"
    pub fn collides(&self, triangle: (&Vec2, &Vec2, &Vec2), position: Vec2, rotation: f32) -> bool {
        let (p1, p2, p3) = (
            rotate(*triangle.0, rotation) + position,
            rotate(*triangle.1, rotation) + position,
            rotate(*triangle.2, rotation) + position,
        );
        if p1.x < -1.0 || p2.x < -1.0 || p3.x < -1.0 {
            return true;
        }
        if p1.x >= 1.0 || p2.x >= 1.0 || p3.x >= 1.0 {
            return true;
        }
        if p1.y < -1.0 || p2.y < -1.0 || p3.y < -1.0 {
            return true;
        }
        if p1.y >= 1.0 || p2.y >= 1.0 || p3.y >= 1.0 {
            return true;
        }
        let nearby_cells = self.nearby_cells(position);
        let possible_colliders: Vec<(Vec2, Vec2, Vec2)> = nearby_cells
            .into_iter()
            .map(|(x, y)| {
                (
                    rotate(self.drawable.verts[0], self.rotations[y][x]) + self.positions[y][x],
                    rotate(self.drawable.verts[1], self.rotations[y][x]) + self.positions[y][x],
                    rotate(self.drawable.verts[2], self.rotations[y][x]) + self.positions[y][x],
                )
            })
            .collect();
        for coll in possible_colliders {
            if collide((&p1, &p2, &p3), (&coll.0, &coll.1, &coll.2)) {
                return true;
            }
        }
        false
    }
}

impl GameObject for Labirynth {
    fn draw(
        &mut self,
        ctx: &microengine::context::Context,
        _scene: &microengine::Scene,
    ) -> microengine::error::GameResult {
        self.drawable.draw(ctx.window.aspect_ratio() as f32);
        Ok(())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
