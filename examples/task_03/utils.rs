#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }
}

use glm::{Mat2, Vec2};
use nalgebra_glm::Vec3;

pub fn rotate(point: Vec2, angle: f32) -> Vec2 {
    Mat2::new(angle.cos(), -angle.sin(), angle.sin(), angle.cos()) * point
}

/// Check if points p1 and p2 lay on the same side of ab
fn same_side(p1: &Vec2, p2: &Vec2, a: &Vec2, b: &Vec2) -> bool {
    let v1 = b - a;
    let v1 = Vec3::new(v1.x, v1.y, 0.0);
    let v2 = p1 - a;
    let v2 = Vec3::new(v2.x, v2.y, 0.0);
    let v3 = p2 - a;
    let v3 = Vec3::new(v3.x, v3.y, 0.0);
    let cp1 = glm::cross(&v1, &v2);
    let cp2 = glm::cross(&v1, &v3);
    if glm::dot(&cp1, &cp2) >= 0.0 {
        true
    } else {
        false
    }
}

/// Check if point is in a triangle
fn point_in_triangle(p: &Vec2, tri: (&Vec2, &Vec2, &Vec2)) -> bool {
    let (a, b, c) = (tri.0, tri.1, tri.2);
    if same_side(p, a, b, c) && same_side(p, b, a, c) && same_side(p, c, a, b) {
        true
    } else {
        false
    }
}

pub fn collide(tri_1: (&Vec2, &Vec2, &Vec2), tri_2: (&Vec2, &Vec2, &Vec2)) -> bool {
    point_in_triangle(tri_1.0, tri_2)
        || point_in_triangle(tri_1.1, tri_2)
        || point_in_triangle(tri_1.2, tri_2)
        || point_in_triangle(tri_2.0, tri_1)
        || point_in_triangle(tri_2.1, tri_1)
        || point_in_triangle(tri_2.2, tri_1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collide_triangles_true() {
        let tri1 = (
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(0.0, 2.0),
        );
        let tri2 = (
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(1.0, 1.0),
        );
        assert!(collide(
            (&tri1.0, &tri1.1, &tri1.2),
            (&tri2.0, &tri2.1, &tri2.2)
        ));
    }
    #[test]
    fn collide_triangles_false() {
        let tri1 = (
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(0.0, 2.0),
        );
        let tri2 = (
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(-1.0, -1.0),
        );
        assert!(!collide(
            (&tri1.0, &tri1.1, &tri1.2),
            (&tri2.0, &tri2.1, &tri2.2)
        ));
    }
}
