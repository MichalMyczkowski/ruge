#version 330 
layout(location = 0) in vec2 vert;
layout(location = 1) in vec2 position;
layout(location = 2) in float rotation;

uniform float aspect_ratio;
uniform int last_instance;
out vec4 instance_color;

mat2 rotate(float angle) {
    return mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
}

mat2 scale(vec2 scale) {
    return mat2(scale.x, 0, 0, scale.y);
}

void main(void) {
    vec2 scale_factor = (aspect_ratio > 1) ? vec2(1.0, 1.0/aspect_ratio) : vec2(aspect_ratio, 1.0);
    vec2 position = (vert * rotate(rotation + radians(30))) + position;
    position *= scale(scale_factor); 
    gl_Position = vec4(position, 0.0, 1.0);

    if (gl_InstanceID > 10) {
        instance_color = vec4(0.0, 0.2, 0.5, 1.0);
    }
    if (gl_InstanceID > 20) {
        instance_color = vec4(0.0, 1.0, 0.5, 1.0);
    } else {
        instance_color = vec4(1.0, 0.0, 0.5, 1.0);
    }

    if (gl_InstanceID == last_instance || gl_InstanceID == 0) {
        instance_color.a = 0.0;
    }

}
