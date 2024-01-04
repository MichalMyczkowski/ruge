#version 330 
uniform vec3 color;
in vec4 instance_color;
out vec4 out_color;

void main(void) {
        out_color = instance_color;
} 
