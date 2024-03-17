#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 projection;
uniform vec3 viewer_pos;

out vec3 frag_pos;
out vec3 frag_normal;
out vec4 cam_pos;

void main(void) {
    mat4 mvp = projection * model;
    gl_Position = mvp * vec4(vert, 1.0);
    frag_pos = vec3(model * vec4(vert, 1.0));
    frag_normal = mat3(transpose(inverse(model))) * normal * -1.0;
    cam_pos = vec4(viewer_pos, 1.0);
}
