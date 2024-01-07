#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec4 mvp1;
layout(location = 2) in vec4 mvp2;
layout(location = 3) in vec4 mvp3;
layout(location = 4) in vec4 mvp4;
layout(location = 5) in float color;

//uniform float colors[80];
//uniform mat4 mvps[83];
uniform float radius;
uniform float time;

out float texture_coord_x;

void main(void) {
   mat4 mvp = mat4(mvp1, mvp2, mvp3, mvp4);   
   gl_Position = mvp * vec4(vert, 1.0);
   //gl_Position = mvps[gl_InstanceID] * vec4(vert, 1.0);
   //texture_coord_x = colors[gl_InstanceID];
   texture_coord_x = color;
}

