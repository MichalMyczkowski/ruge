use std::iter::{self, repeat};

pub struct Plane {
    pub verts: Vec<glm::Vec3>,
    pub indices: Vec<u32>,
    pub normals: Vec<glm::Vec3>,
}

impl Plane {
    
    fn get_idx(row: usize, column: usize, size: usize) -> u32 {
        (column + size * row) as u32
    }

    /// Creates plane mesh with given subdivision_level
    /// subdivision_level = 0 means no subdivision
    pub fn new(subdivision_level: usize) -> Self {
        // calculate vertices
        let vert_capacity = 2 + subdivision_level;
        let side = 2.0f32;
        let subdivision = subdivision_level as f32 + 1.0;
        let offset = side / subdivision;
        let mut x = -1.0; 
        let verts = iter::repeat_with(|| {
            let mut z = -1.0;
            let v = iter::repeat_with(|| {
                let v = glm::Vec3::new(x, 0.0, z);
                z += offset;
                v
            }).take(vert_capacity).collect::<Vec<glm::Vec3>>();
            x += offset;
            v
        }).take(vert_capacity).collect::<Vec<Vec<glm::Vec3>>>();
        
        // calculate indices
        let mut indices: Vec<u32> = Vec::with_capacity((vert_capacity - 1).pow(2) * 2);
        for row in 0..verts.len() - 1 {
            for column in 0..verts[row].len() - 1 {
                // first triangle
                indices.push(Self::get_idx(row, column, vert_capacity));
                indices.push(Self::get_idx(row, column + 1, vert_capacity));
                indices.push(Self::get_idx(row + 1, column, vert_capacity));
                // second triangle
                indices.push(Self::get_idx(row + 1, column, vert_capacity));
                indices.push(Self::get_idx(row, column + 1, vert_capacity));
                indices.push(Self::get_idx(row + 1, column + 1, vert_capacity));
            }
        }

        let verts = verts.into_iter().flatten().collect::<Vec<glm::Vec3>>();
        let normals = repeat(glm::Vec3::y()).take(verts.len()).collect::<Vec<glm::Vec3>>();
        Self {
            verts,
            normals,
            indices,
        }
    }
}
