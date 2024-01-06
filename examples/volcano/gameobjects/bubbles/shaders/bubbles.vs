#version 330 
layout(location = 0) in vec3 vert;

uniform float colors[80];
uniform mat4 mvps[83];
uniform float radius;
uniform float time;

out float texture_coord_x;

void main(void) {
   gl_Position = mvps[gl_InstanceID] * vec4(vert, 1.0);
   texture_coord_x = colors[gl_InstanceID];
}

