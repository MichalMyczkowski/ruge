#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec3 normal;

uniform mat4 model;
uniform mat4 projection;
uniform vec3 viewer_pos;

out vec3 frag_pos;
out vec3 frag_normal;
out vec4 cam_pos;

out vec4 v_colour;
out float texture_coord_x;

void main(void) {
   gl_Position = projection * model * vec4(vert, 1.0);
   texture_coord_x = (vert.y + 2.0) / 4.0 + 0.2;
   v_colour = vec4(0.6, 0.3, 0.742, 1.0);

   frag_pos = vec3(model * vec4(vert, 1.0));
   frag_normal = mat3(transpose(inverse(model))) * normal;  
   cam_pos = vec4(viewer_pos, 1.0);

}

