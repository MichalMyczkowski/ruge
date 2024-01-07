#version 330 
in vec4 v_colour;
in vec2 t_coords;
uniform sampler2D gradient;
out vec4 out_color;

void main(void) {
    out_color = texture(gradient, t_coords);
} 

