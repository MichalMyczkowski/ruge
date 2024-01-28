#version 330 
layout(location = 0) in vec3 vert;

uniform mat4 mvp;
uniform float radius;

void main(void) {
   gl_Position = mvp * vec4(vert * radius, 1.0);
}