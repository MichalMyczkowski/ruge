#version 330 
layout(location = 0) in vec3 vert;

uniform mat4 mvp;
uniform float radius;

out vec4 fragpos;

void main(void) {
    gl_Position = mvp * vec4(radius * vert, 1.0);
}
