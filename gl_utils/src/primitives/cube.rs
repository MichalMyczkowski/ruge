use std::iter;

pub struct Cube {
    pub verts: Vec<glm::Vec3>,
    pub indices: Vec<u32>,
    pub texture_coordinates: Vec<glm::Vec2>,
    pub normals: Vec<glm::Vec3>,
}

impl Cube {
    pub fn new() -> Self {
        let mut verts = vec![
            // #1
            glm::Vec3::new(0.0, 0.0, 0.0),
            glm::Vec3::new(0.0, 1.0, 0.0),
            glm::Vec3::new(1.0, 1.0, 0.0),
            glm::Vec3::new(1.0, 0.0, 0.0),
            // #2
            glm::Vec3::new(1.0, 0.0, 0.0),
            glm::Vec3::new(1.0, 1.0, 0.0),
            glm::Vec3::new(1.0, 1.0, -1.0),
            glm::Vec3::new(1.0, 0.0, -1.0),
            // #3
            glm::Vec3::new(1.0, 0.0, -1.0),
            glm::Vec3::new(1.0, 1.0, -1.0),
            glm::Vec3::new(0.0, 1.0, -1.0),
            glm::Vec3::new(0.0, 0.0, -1.0),
            // #4
            glm::Vec3::new(0.0, 0.0, -1.0),
            glm::Vec3::new(0.0, 1.0, -1.0),
            glm::Vec3::new(0.0, 1.0, 0.0),
            glm::Vec3::new(0.0, 0.0, 0.0),
            // #5
            glm::Vec3::new(0.0, 1.0, 0.0),
            glm::Vec3::new(0.0, 1.0, -1.0),
            glm::Vec3::new(1.0, 1.0, -1.0),
            glm::Vec3::new(1.0, 1.0, 0.0),
            // #6
            glm::Vec3::new(0.0, 0.0, -1.0),
            glm::Vec3::new(0.0, 0.0, 0.0),
            glm::Vec3::new(1.0, 0.0, 0.0),
            glm::Vec3::new(1.0, 0.0, -1.0),
        ];
        let normals = vec![
            // #1
            glm::Vec3::new(0.0, 0.0, 1.0),
            glm::Vec3::new(0.0, 0.0, 1.0),
            glm::Vec3::new(0.0, 0.0, 1.0),
            glm::Vec3::new(0.0, 0.0, 1.0),
            // #2
            glm::Vec3::new(1.0, 0.0, 0.0),
            glm::Vec3::new(1.0, 0.0, 0.0),
            glm::Vec3::new(1.0, 0.0, 0.0),
            glm::Vec3::new(1.0, 0.0, 0.0),
            // #3
            glm::Vec3::new(0.0, 0.0, -1.0),
            glm::Vec3::new(0.0, 0.0, -1.0),
            glm::Vec3::new(0.0, 0.0, -1.0),
            glm::Vec3::new(0.0, 0.0, -1.0),
            // #4
            glm::Vec3::new(-1.0, 0.0, 0.0),
            glm::Vec3::new(-1.0, 0.0, 0.0),
            glm::Vec3::new(-1.0, 0.0, 0.0),
            glm::Vec3::new(-1.0, 0.0, 0.0),
            // #5
            glm::Vec3::new(0.0, 1.0, 0.0),
            glm::Vec3::new(0.0, 1.0, 0.0),
            glm::Vec3::new(0.0, 1.0, 0.0),
            glm::Vec3::new(0.0, 1.0, 0.0),
            // #6
            glm::Vec3::new(0.0, -1.0, 0.0),
            glm::Vec3::new(0.0, -1.0, 0.0),
            glm::Vec3::new(0.0, -1.0, 0.0),
            glm::Vec3::new(0.0, -1.0, 0.0),
        ];
        verts.iter_mut().for_each(|v| {
            v.x -= 0.5;
            v.y -= 0.5;
            v.z += 0.5;
        });
        let mut idx = 0;
        let indices = iter::repeat_with(|| {
            let v = vec![2 + idx, 1 + idx, 0 + idx, 0 + idx, 3 + idx, 2 + idx];
            idx += 4;
            v
        }).take(6).flatten().collect::<Vec<u32>>();
        let texture_coordinates = iter::repeat(
            vec![
                glm::Vec2::new(1.0, 0.0),
                glm::Vec2::new(0.0, 0.0),
                glm::Vec2::new(0.0, 1.0),
                glm::Vec2::new(1.0, 1.0),
            ]
        ).take(6).flatten().collect::<Vec<glm::Vec2>>();

        Self {
            verts,
            indices,
            normals,
            texture_coordinates,
        }
    }
}
