#version 330 

uniform int full;
out vec4 out_color;

void main(void) {
    if (full == 1) {
        out_color = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        out_color = vec4(0.0, 0.0, 0.0, 1.0);
    }
} 
