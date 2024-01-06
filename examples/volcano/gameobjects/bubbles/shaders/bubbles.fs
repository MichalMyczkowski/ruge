#version 330 
in float texture_coord_x;
uniform sampler2D gradient;

out vec4 out_color;

void main(void) {
    out_color = texture(gradient, vec2(texture_coord_x, 0.0));
    out_color.a = 0.2;
} 

