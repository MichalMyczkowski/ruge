

// Code adapted from embree as user 'Fedor' suggested:
// https://stackoverflow.com/questions/2924795/fastest-way-to-compute-point-to-triangle-distance-in-3d

pub fn closestPointTriangle(p: &glm::Vec3, a: &glm::Vec3, b: &glm::Vec3, c: &glm::Vec3) -> glm::Vec3 {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;

    let d1 = glm::dot(&ab, &ap);
    let d2 = glm::dot(&ac, &ap);
    if d1 <= 0f32 && d2 <= 0f32 { 
        // #1
        return a.clone();
    }

    let bp = p - b;
    let d3 = glm::dot(&ab, &bp);
    let d4 = glm::dot(&ac, &bp);

    if d3 >= 0f32 && d4 <= d3 { 
        // #2
        return b.clone();
    }

    let cp = p - c;
    let d5 = glm::dot(&ab, &cp);
    let d6 = glm::dot(&ac, &cp);

    if d6 >= 0f32 && d5 <= d6 {
        // #3
        return c.clone();
    }

    let vc = d1 * d4 - d3 * d2;
    if vc <= 0f32 && d1 >= 0f32 && d3 <= 0f32 {
        // #4
        let v = d1 / (d1 - d3);
        return a + v * ab;
    }

    let vb = d5 * d2 - d1 * d6;
    if vb <= 0f32 && d2 >= 0f32 && d6 <= 0f32 {
        // #5
        let v = d2 / (d2 - d6);
        return a + v * ac;
    }

    let va = d3 * d6 - d5 * d4;
    if va <= 0f32 && (d4 - d3) >= 0f32 && (d5 - d6) >= 0f32 {
        // #6
        let v = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return b + v * (c - b);
    }

    // #0
    let denom = 1f32 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    return a + v * ab + w * ac;
}
