#version 330 
in vec2 texture_coord;
in vec4 color;

uniform sampler2D bg_texture;
out vec4 out_color;


void main(void) {
    out_color = texture(bg_texture, texture_coord);
    out_color *= color;
    out_color.a = 1.0;
} 

