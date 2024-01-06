#version 330 
in vec2 texture_coord;
in vec4 color;
in float instance_id;

uniform int last_instance;
uniform sampler2D bg_texture;
out vec4 out_color;


void main(void) {
    if (instance_id == 0.0 || instance_id == float(last_instance)) {
        discard;
    }
    out_color = texture(bg_texture, texture_coord);
    out_color *= color;
    out_color.a = 1.0;
} 

