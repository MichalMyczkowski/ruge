#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec2 texture_coords;
layout(location = 2) in vec3 normals;

uniform float coord_offset;
uniform float vert_offset;
uniform mat4 mvp;
uniform int full;
uniform float time;
uniform sampler2D gradient;
out vec4 normal;
out vec4 v_colour;
out float texture_coord_x;
out vec2 tc;

vec3 heightmap(vec3 verti, vec2 coords) {
    float height = texture(gradient, vec2(coords.x, coords.y - time * 0.05)).x;
    if (full == 0) {
        height += 0.001;
    }
    return vec3(verti.x, verti.y + height * 15.0, verti.z);
}

vec3 calculate_normal() {
    // surrounding vertices height
    vec3 c = heightmap(
        vec3(vert.x, vert.y, vert.z),
        vec2(texture_coords.x, texture_coords.y)
    ); 
    vec3 rb = heightmap(
        vec3(vert.x + vert_offset, vert.y, vert.z - vert_offset),
        vec2(texture_coords.x + coord_offset, texture_coords.y - coord_offset)
    ); 
    vec3 r = heightmap(
        vec3(vert.x + vert_offset, vert.y, vert.z),
        vec2(texture_coords.x + coord_offset, texture_coords.y)
    ); 
    vec3 rt = heightmap(
        vec3(vert.x + vert_offset, vert.y, vert.z + vert_offset),
        vec2(texture_coords.x + coord_offset, texture_coords.y + coord_offset)
    ); 
    vec3 t = heightmap(
        vec3(vert.x, vert.y, vert.z + vert_offset),
        vec2(texture_coords.x, texture_coords.y + coord_offset)
    ); 
    vec3 lt = heightmap(
        vec3(vert.x - vert_offset, vert.y, vert.z + vert_offset),
        vec2(texture_coords.x - coord_offset, texture_coords.y + coord_offset)
    ); 
    vec3 l = heightmap(
        vec3(vert.x - vert_offset, vert.y, vert.z),
        vec2(texture_coords.x - coord_offset, texture_coords.y)
    ); 
    vec3 lb = heightmap(
        vec3(vert.x - vert_offset, vert.y, vert.z - vert_offset),
        vec2(texture_coords.x - coord_offset, texture_coords.y - coord_offset)
    ); 
    vec3 b = heightmap(
        vec3(vert.x, vert.y, vert.z - vert_offset),
        vec2(texture_coords.x, texture_coords.y - coord_offset)
    ); 
    vec3 normal = vec3(0.0);
    normal += abs(cross(c - rt, r - rt));
    normal += abs(cross(c - t, rt - t));
    normal += abs(cross(c - lt, t - lt));
    normal += abs(cross(c - l, lt - l));
    normal += abs(cross(c - lb, l - lb));
    normal += abs(cross(c - b, lb - b));
    normal += abs(cross(c - rb, b - rb));
    normal += abs(cross(c - r, rb - r));
    return normalize(normal);
}


void main(void) {
   gl_Position = mvp * vec4(heightmap(vert, texture_coords), 1.0);
   texture_coord_x = (vert.y + 2.0) / 4.0 + 0.2;
   v_colour = vec4(0.6, 0.3, 0.742, 1.0);
   tc = texture_coords;

   normal = vec4(calculate_normal(), 0.0);
}

