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
            self.segments * (self.index - self.closed as usize) + self.closed as usize
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
    pub fn new(segments: usize, rings: usize) -> Self {
        let s = SolidOfRevolution::new(2.0, segments, rings + 2, false, |x| (1.0f32 - x.powi(2)).sqrt());
        Self {
            verts: s.verts,
            indices: s.indices,
        }
    }
}


pub struct SolidOfRevolution {
    pub verts: Vec<glm::Vec3>,
    pub indices: Vec<u32>,
} 

impl SolidOfRevolution {

    // TODO: Return GameResult<Self> instead of 'fixing' errors
    pub fn new<T>(height: f32, mut segments: usize, mut rings: usize, closed: bool, mut f: T) -> Self
    where
        T: FnMut(f32) -> f32,
    {
        if segments < 3 {
            segments = 3;
        }
        if rings < 2 {
            rings = 2;
        }
        let indice_capacity;
        let dividers = rings - 1;
        if closed {
            indice_capacity = 2 * 3 * rings * segments;
        } else {
            indice_capacity = 2 * 3 * (rings - 1) * segments;
        }

        let mut indices: Vec<u32> = Vec::with_capacity(indice_capacity);
        let mut ring_v: Vec<Ring> = Vec::with_capacity(rings + 2 * closed as usize);

        let y_spacing = (height) / (dividers) as f32;
        let mut y = -height / 2.0;
        let mut idx = 0usize;

        // Calculate verts
        if closed {
            ring_v.push(Ring::new_point_ring(glm::Vec3::new(0.0, y, 0.0), segments, idx, closed));
            idx += 1;
        }
        for _ in 0..rings {
            // TODO! return error if radius = NAN or inf
            let radius = f(y);
            ring_v.push(Ring::new(radius, segments, y, idx, closed));
            idx += 1;
            y += y_spacing;
        }
        if closed {
            y -= y_spacing;
            ring_v.push(Ring::new_point_ring(glm::Vec3::new(0.0, y, 0.0), segments, idx, closed));
        }

        // each point on each ring creates two triangles
        for r_idx in 0..ring_v.len() {
            for seg_idx in 0..segments {
                if closed && r_idx == 0 { break; }
                if closed && r_idx == ring_v.len() - 1 { break; }
                // first triangle
                if r_idx != 0 {
                    indices.push(ring_v[r_idx].get_vert_idx(seg_idx) as u32);
                    indices.push(ring_v[r_idx].get_vert_idx(seg_idx + 1) as u32);
                    indices.push(ring_v[r_idx - 1].get_vert_idx(seg_idx + 1) as u32);
                }
                // second triangle
                if r_idx != ring_v.len() - 1 {
                    indices.push(ring_v[r_idx].get_vert_idx(seg_idx) as u32);
                    indices.push(ring_v[r_idx + 1].get_vert_idx(seg_idx) as u32);
                    indices.push(ring_v[r_idx].get_vert_idx(seg_idx + 1) as u32);
                }
            }
        }

        Self {
            verts: ring_v.into_iter().flat_map(|r| r.points).collect(),
            indices,
        }
    }
}
