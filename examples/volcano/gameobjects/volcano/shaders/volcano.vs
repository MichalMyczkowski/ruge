#version 330 
layout(location = 0) in vec3 vert;

uniform mat4 mvp;
out vec4 v_colour;
out float texture_coord_x;

void main(void) {
   gl_Position = mvp * vec4(vert, 1.0);
   texture_coord_x = (vert.y + 2.0) / 4.0 + 0.2;
   v_colour = vec4(0.6, 0.3, 0.742, 1.0);
}

