#version 330 
uniform float time;
uniform float size;

in vec2 center;
out vec4 out_color;

float fromCenter(vec2 pos){
   return sqrt(pos.x*pos.x + pos.y*pos.y);
}

void main(void) {
    float value = 1.0 - abs(sin(time/2.0))/1.2 - (3.0 * fromCenter(center))/(size);
    if (value < 0.0) {
        value = 0.0;
    }
    out_color = vec4(1.0, 1.0, 1.0, value);
} 
