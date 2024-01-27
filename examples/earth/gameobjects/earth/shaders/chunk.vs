#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec2 texture_coords;

uniform float radius;
uniform float mix_val;
uniform vec3 scale_vec;
uniform vec3 start_offset;
uniform mat4 projection;
uniform int full;

void main(void) {
    // default chunk positioning
    vec3 pos = vert + start_offset;
    pos.x *= scale_vec.x;
    pos.y *= scale_vec.y;
    pos.z *= scale_vec.z;

    // mapping to sphere
    float dist = distance(pos, vec3(0.0));
    pos = normalize(pos);
    float r = radius;
    if (full == 0) {
        r *= 1.001;
    }
    pos *= mix(r, dist, mix_val);
    gl_Position = projection * vec4(pos, 1.0);
}

