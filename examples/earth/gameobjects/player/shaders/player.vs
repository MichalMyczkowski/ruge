#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec2 texture_coord;
layout(location = 2) in vec3 normal;

uniform mat4 model;
uniform mat4 projection;
uniform vec3 viewer_pos;
uniform mat4 tail_model;
uniform mat4 blade1;
uniform mat4 blade2;
out vec2 t_coords;

out vec3 frag_pos;
out vec3 frag_normal;
out vec4 cam_pos;

void main(void) {
    mat4 mvp = projection * model;
    if (gl_InstanceID == 0) {
        gl_Position = mvp * vec4(vert, 1.0);
        frag_pos = vec3(model * vec4(vert, 1.0));
        frag_normal = mat3(transpose(inverse(model))) * normal * -1.0;
    } else if (gl_InstanceID == 1) {
        gl_Position = mvp * blade1 * vec4(vert, 1.0);
        frag_pos = vec3(model * blade1 * vec4(vert, 1.0));
        frag_normal = mat3(transpose(inverse(model * blade1))) * normal * -1.0;
    } else if (gl_InstanceID == 2) {
        gl_Position = mvp * blade2 * vec4(vert, 1.0);
        frag_pos = vec3(model * blade2 * vec4(vert, 1.0));
        frag_normal = mat3(transpose(inverse(model * blade2))) * normal * -1.0;
    } else {
        gl_Position = mvp * tail_model * vec4(vert, 1.0);
        frag_pos = vec3(model * tail_model * vec4(vert, 1.0));
        frag_normal = mat3(transpose(inverse(model * tail_model))) * normal * -1.0;
    }
   t_coords = texture_coord;
   cam_pos = vec4(viewer_pos, 1.0);
}
