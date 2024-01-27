#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec4 colour;

uniform mat4 mvp;

out vec4 v_colour;

void main(void) {
   gl_Position = mvp * vec4(vert, 1.0);
   v_colour = colour;
}
