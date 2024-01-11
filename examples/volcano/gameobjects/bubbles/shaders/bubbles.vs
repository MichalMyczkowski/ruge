#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 model1;
layout(location = 3) in vec4 model2;
layout(location = 4) in vec4 model3;
layout(location = 5) in vec4 model4;
layout(location = 6) in float color;

uniform mat4 projection;
uniform vec3 viewer_pos;
uniform float radius;
uniform float time;

out float texture_coord_x;
out vec3 frag_pos;
out vec3 frag_normal;
out vec4 cam_pos;

void main(void) {
   mat4 model = mat4(model1, model2, model3, model4);   
   gl_Position = projection * model * vec4(vert, 1.0);
   frag_pos = vec3(model * vec4(vert, 1.0));
   texture_coord_x = color;
   frag_normal = mat3(transpose(inverse(model))) * normal;  
   cam_pos = vec4(viewer_pos, 1.0);

}

