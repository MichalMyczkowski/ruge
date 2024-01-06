use std::iter;

/// Helping structure
struct Ring {
    index: usize,
    segments: usize,
    points: Vec<glm::Vec3>,
    closed: bool,
}

impl Ring {
    /// Use to create first or last ring (if figure is closed)
    pub fn new_point_ring(point: glm::Vec3, segments: usize, index: usize, closed: bool) -> Self {
        Self {
            index,
            segments,
            points: vec![point],
            closed,
        }

    }
    pub fn new(radius: f32, segments: usize, y: f32, idx: usize, closed: bool) -> Self {
        let mut value = 0.0f32;
        let step = std::f32::consts::PI * 2.0 / segments as f32;
        let points = iter::repeat_with(|| {
            let v = glm::Vec3::new(value.cos() * radius, y, value.sin() * radius);
            value += step;
            v
        }).take(segments).collect();

        Self {
            index: idx,
            segments,
            points,
            closed,
        }
    }
    
    /// returns in-mesh index of idx-th vertex in ring
    pub fn get_vert_idx(&self, idx: usize) -> usize {
        let before = if self.index == 0  {
            0
        } else {
            self.segments * (self.index - 1) + self.closed as usize
        };
        let idx = idx % self.points.len();
        idx + before
    }
}

pub struct Sphere {
    pub verts: Vec<glm::Vec3>,
    pub indices: Vec<u32>,
}

impl Sphere {
    pub fn new(radius: f32, mut segments: usize, mut rings: usize) -> Self {
        if segments < 3 {
            segments = 3;
        }
        if rings == 0 {
            rings = 1;
        }

        let mut indices: Vec<u32> = Vec::with_capacity(3 * rings * segments * 2);
        let mut idx = 0usize;
        let y_spacing = (2.0 * radius) / (rings + 1) as f32;

        let rings: Vec<Ring> = iter::repeat_with(|| {
            let r;
            if idx == 0 {
                r = Ring::new_point_ring(glm::vec3(0.0, -radius, 0.0), segments, idx, true);
            } else if idx == rings + 1 {
                r = Ring::new_point_ring(glm::vec3(0.0, radius, 0.0), segments, idx, true);
            } else {
                let y = (y_spacing * idx as f32) - radius;
                let radius = f32::sqrt(radius.powi(2) - y.powi(2));
                r = Ring::new(radius, segments, y, idx, true);
            }
            idx += 1;
            r
        }).take(rings + 2).collect();

        // each point on each ring creates two triangles
        for r_idx in 1..rings.len()-1 {
            for seg_idx in 0..segments {
                // first triangle
                indices.push(rings[r_idx].get_vert_idx(seg_idx) as u32);
                indices.push(rings[r_idx].get_vert_idx(seg_idx + 1) as u32);
                indices.push(rings[r_idx - 1].get_vert_idx(seg_idx + 1) as u32);
                // second triangle
                indices.push(rings[r_idx].get_vert_idx(seg_idx) as u32);
                indices.push(rings[r_idx + 1].get_vert_idx(seg_idx) as u32);
                indices.push(rings[r_idx].get_vert_idx(seg_idx + 1) as u32);
            }
        }

        Sphere {
            verts: rings.into_iter().flat_map(|r| r.points).collect(),
            indices,
        }
    }
}
