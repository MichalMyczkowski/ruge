#version 330 
layout(location = 0) in vec2 vert;
layout(location = 1) in vec2 texture_coord;

uniform vec2  position;
uniform float rotation;
uniform float aspect_ratio;

out vec2 tex_coord;

mat2 rotate(float angle) {
    return mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
}

mat2 scale(vec2 scale) {
    return mat2(scale.x, 0, 0, scale.y);
}

void main(void) {
   vec2 scale_factor = (aspect_ratio > 1) ? vec2(1.0, 1.0/aspect_ratio) : vec2(aspect_ratio, 1.0);
   vec2 position = vert + position;
   position *= scale(scale_factor); 
   tex_coord = texture_coord;
   gl_Position = vec4(position, 0.0, 1.0);
}
