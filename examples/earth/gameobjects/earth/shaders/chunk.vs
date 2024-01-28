#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec2 texture_coords;

uniform float radius;
uniform float mix_val;
// -- 
uniform vec3 start_pos;
uniform vec3 up;
uniform vec3 right;
uniform float width;
// --
uniform mat4 projection;
// --
uniform int full;

void main(void) {
    // default chunk positioning
    vec3 pos = start_pos + 
        (vert.z * up * width) +
        (vert.x * right * width);

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

