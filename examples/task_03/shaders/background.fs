#version 330 
in vec2 tex_coord;

uniform float time;
uniform sampler2D bg_texture;

out vec4 out_color;

float fromCenter(vec2 pos){
   return sqrt(pos.x*pos.x + pos.y*pos.y);
}

void main(void) {
    highp int index_x = int(gl_FragCoord.x);
    highp int index_y = int(gl_FragCoord.y);
    float alpha = 0.5 - fromCenter(tex_coord - 0.5);
    vec2 tex_coord = vec2(tex_coord.x + sin(time / 20.0), tex_coord.y + sin(time / 20.0));
    if (index_y % 10 > 4) {
        out_color = texture(bg_texture, vec2(tex_coord.x + sin(time/5.0)/20.0, tex_coord.y));
    } else {
        out_color = texture(bg_texture, vec2(tex_coord.x - sin(time/5.0)/20.0, tex_coord.y));
    }
    out_color.a *= alpha;
} 
