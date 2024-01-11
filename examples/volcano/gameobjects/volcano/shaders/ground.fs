#version 330 
in vec4 v_colour;
in vec2 tc;
in vec4 normal;
in float texture_coord_x;
uniform sampler2D gradient;
uniform float time;
uniform int full;
out vec4 out_color;

void main(void) {
    if (full == 1) {
        //out_color = texture(gradient, tc) * vec4(0.513, 0.356, 0.69, 1.0);
        vec4 normall = normal;
        normall.a = 1.0;
        out_color = normall;
    } else {
        out_color = vec4(0.864, 0.153, 0.4124, 1.0) * 0.78;
    }

} 
