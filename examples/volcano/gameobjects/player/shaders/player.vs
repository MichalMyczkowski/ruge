#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec2 texture_coord;

uniform mat4 mvp;
uniform mat4 tail_model;
uniform mat4 blade1;
uniform mat4 blade2;

out vec4 v_colour;
out vec2 t_coords;

void main(void) {
    if (gl_InstanceID == 0) {
        gl_Position = mvp * vec4(vert, 1.0);
    } else if (gl_InstanceID == 1) {
        gl_Position = mvp * blade1 * vec4(vert, 1.0);
    } else if (gl_InstanceID == 2) {
        gl_Position = mvp * blade2 * vec4(vert, 1.0);
    } else {
        gl_Position = mvp * tail_model * vec4(vert, 1.0);
    }
   v_colour = vec4(0.6, 0.3, 0.742, 1.0);
   t_coords = texture_coord;
}
