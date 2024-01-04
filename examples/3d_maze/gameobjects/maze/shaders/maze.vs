#version 330 
layout(location = 0) in vec3 vert;
layout(location = 1) in vec2 tex_coord;
layout(location = 2) in vec4 m_1;
layout(location = 3) in vec4 m_2;
layout(location = 4) in vec4 m_3;
layout(location = 5) in vec4 m_4;

uniform int last_instance;
uniform int size;
uniform mat4 projection;

out vec2 texture_coord;
out vec3 position;
out vec4 color;

void main(void) {
    mat4 model = mat4(m_1, m_2, m_3, m_4);
    gl_Position = projection * model * vec4(vert, 1.0);
    texture_coord = tex_coord;
    int squared = size * size;
    int x = gl_InstanceID % size;
    int z = ((gl_InstanceID % squared) / size);
    int y = gl_InstanceID / squared;
    float xx = float(x)/float(size);
    float zz = float(z)/float(size);
    float yy = float(y)/float(size);
    color = vec4(xx, zz, yy, 1.0);
    position = vec3(xx, yy, zz);

    //if (gl_InstanceID == last_instance || gl_InstanceID == 0) {
    //    color.a = 0.0;
    //}

}
