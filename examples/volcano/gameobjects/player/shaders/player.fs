#version 330 
in vec4 v_colour;
in float texture_coord_x;
uniform sampler2D gradient;
uniform float time;
out vec4 out_color;

void main(void) {
    out_color = texture(gradient, vec2(texture_coord_x * (1.0 + sin(time) * 0.2), 0.0));
} 

