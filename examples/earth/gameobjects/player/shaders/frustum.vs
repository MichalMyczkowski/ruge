#version 330 
layout(location = 0) in vec3 vert;

uniform mat4 mvp;
uniform mat4 inv_projection;

void main(void) {
   gl_Position = mvp * inv_projection * vec4(vert, 1.0);
}
