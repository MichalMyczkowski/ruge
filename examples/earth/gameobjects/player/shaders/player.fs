#version 330 
// TODO! import light data with gl_utils preprocessor (not yet implemented)
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
}; 

struct LightColor {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
};

struct DirLight {
    vec4 direction;
    LightColor color;
};  

struct PointLight {
    vec4 position;
    LightColor color;
    float constant;
    float linear;
    float quadratic;
    float _padding1;
}; 

struct SpotLight {
    vec4  position;
    vec4 direction;
    LightColor color;
    float cutoff;
    float outer_cutoff;
    float _padding2;
    float _padding3;
};

float calculate_specular(vec3 viewDir, vec3 normal, vec3 lightDir, float shininess) {
     // Calculate the cosine of the angle between the reflection vector
     // and the vector going to the camera.
     vec3 reflection = normalize(2.0 * dot(normal, lightDir) * normal - lightDir);
     viewDir = normalize( -viewDir );
     float cos_angle = dot(reflection, -viewDir);
     cos_angle = clamp(cos_angle, 0.0, 1.0);
     return pow(cos_angle, shininess);
}

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir, Material material, int idx, int max_idx)
{
    if (idx >= max_idx) {
        return vec3(0.0);
    }
    vec3 lightDir = normalize(vec3(light.direction));
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    float spec = calculate_specular(viewDir, normal, lightDir, material.shininess);
    // combine results
    vec3 ambient  = vec3(light.color.ambient)  * material.ambient;
    vec3 diffuse  = vec3(light.color.diffuse)  * diff * material.diffuse;
    vec3 specular = vec3(light.color.specular) * spec * material.specular;
    return (ambient + diffuse + specular);
} 

vec3 CalcPointLight(PointLight light, vec3 frag_pos, vec3 normal, vec3 viewDir, Material material, int idx, int max_idx)
{
    if (idx >= max_idx) {
        return vec3(0.0);
    }
    float dist = length(vec3(light.position) - frag_pos);
    float attenuation = 1.0 / (light.constant + light.linear * dist + light.quadratic * (dist * dist)); 

    vec3 lightDir = normalize(frag_pos - vec3(light.position));
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    float spec = calculate_specular(viewDir, normal, lightDir, material.shininess);
    // combine results
    vec3 ambient  = vec3(light.color.ambient)  * material.ambient * attenuation;
    vec3 diffuse  = vec3(light.color.diffuse)  * diff * material.diffuse * attenuation;
    vec3 specular = vec3(light.color.specular) * spec * material.specular * attenuation;
    return (ambient + diffuse + specular);
} 

vec3 CalcSpotLight(SpotLight light, vec3 frag_pos, vec3 normal, vec3 viewDir, Material material, int idx, int max_idx)
{
    if (idx >= max_idx) {
        return vec3(0.0);
    }
    vec3 lightDir = normalize(frag_pos - vec3(light.position));
    float theta = dot(lightDir, normalize(vec3(light.direction)));
    float epsilon   = light.cutoff - light.outer_cutoff;
    float intensity = clamp((theta - light.outer_cutoff) / epsilon, 0.0, 1.0);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    float spec = calculate_specular(viewDir, normal, vec3(light.direction), material.shininess);
    // combine results
    vec3 diffuse  = vec3(light.color.diffuse)  * diff * material.diffuse * intensity;
    vec3 specular = vec3(light.color.specular) * spec * material.specular * intensity;
    return (diffuse + specular);
} 

#define MAX_LIGHT_COUNT 16
layout (std140) uniform LightData {
    DirLight dir_lights[MAX_LIGHT_COUNT];
    PointLight point_lights[MAX_LIGHT_COUNT];
    SpotLight spot_lights[MAX_LIGHT_COUNT];
    int dir_light_count;
    int point_light_count;
    int spot_light_count;
    int _padding;
};

vec3 CalculateLights(vec3 frag_pos, vec3 normal, vec3 camera_pos, Material material) {
    vec3 out_color = vec3(0.0);
    vec3 view_dir = normalize(frag_pos - vec3(camera_pos));
    // dynamic uniform indexing was not working so this is the solution
    // calculate directional light
    out_color += CalcDirLight(dir_lights[0], vec3(normal), view_dir, material, 0, dir_light_count);
    out_color += CalcDirLight(dir_lights[1], vec3(normal), view_dir, material, 1, dir_light_count);
    out_color += CalcDirLight(dir_lights[2], vec3(normal), view_dir, material, 2, dir_light_count);
    out_color += CalcDirLight(dir_lights[3], vec3(normal), view_dir, material, 3, dir_light_count);
    out_color += CalcDirLight(dir_lights[4], vec3(normal), view_dir, material, 4, dir_light_count);
    out_color += CalcDirLight(dir_lights[5], vec3(normal), view_dir, material, 5, dir_light_count);
    out_color += CalcDirLight(dir_lights[6], vec3(normal), view_dir, material, 6, dir_light_count);
    out_color += CalcDirLight(dir_lights[7], vec3(normal), view_dir, material, 7, dir_light_count);
    out_color += CalcDirLight(dir_lights[8], vec3(normal), view_dir, material, 8, dir_light_count);
    out_color += CalcDirLight(dir_lights[9], vec3(normal), view_dir, material, 9, dir_light_count);
    out_color += CalcDirLight(dir_lights[10], vec3(normal), view_dir, material, 10, dir_light_count);
    out_color += CalcDirLight(dir_lights[11], vec3(normal), view_dir, material, 11, dir_light_count);
    out_color += CalcDirLight(dir_lights[12], vec3(normal), view_dir, material, 12, dir_light_count);
    out_color += CalcDirLight(dir_lights[13], vec3(normal), view_dir, material, 13, dir_light_count);
    out_color += CalcDirLight(dir_lights[14], vec3(normal), view_dir, material, 14, dir_light_count);
    out_color += CalcDirLight(dir_lights[15], vec3(normal), view_dir, material, 15, dir_light_count);
    // calculate spot lights
    //out_color += CalcSpotLight(spot_lights[0], frag_pos, vec3(normal), view_dir, material, 0, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[1], frag_pos, vec3(normal), view_dir, material, 1, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[2], frag_pos, vec3(normal), view_dir, material, 2, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[3], frag_pos, vec3(normal), view_dir, material, 3, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[4], frag_pos, vec3(normal), view_dir, material, 4, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[5], frag_pos, vec3(normal), view_dir, material, 5, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[6], frag_pos, vec3(normal), view_dir, material, 6, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[7], frag_pos, vec3(normal), view_dir, material, 7, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[8], frag_pos, vec3(normal), view_dir, material, 8, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[9], frag_pos, vec3(normal), view_dir, material, 9, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[10], frag_pos, vec3(normal), view_dir, material, 10, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[11], frag_pos, vec3(normal), view_dir, material, 11, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[12], frag_pos, vec3(normal), view_dir, material, 12, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[13], frag_pos, vec3(normal), view_dir, material, 13, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[14], frag_pos, vec3(normal), view_dir, material, 14, spot_light_count);
    //out_color += CalcSpotLight(spot_lights[15], frag_pos, vec3(normal), view_dir, material, 15, spot_light_count);
    // calculate point lights
    out_color += CalcPointLight(point_lights[0], frag_pos, vec3(normal), view_dir, material, 0, point_light_count);
    out_color += CalcPointLight(point_lights[1], frag_pos, vec3(normal), view_dir, material, 1, point_light_count);
    out_color += CalcPointLight(point_lights[2], frag_pos, vec3(normal), view_dir, material, 2, point_light_count);
    out_color += CalcPointLight(point_lights[3], frag_pos, vec3(normal), view_dir, material, 3, point_light_count);
    out_color += CalcPointLight(point_lights[4], frag_pos, vec3(normal), view_dir, material, 4, point_light_count);
    out_color += CalcPointLight(point_lights[5], frag_pos, vec3(normal), view_dir, material, 5, point_light_count);
    out_color += CalcPointLight(point_lights[6], frag_pos, vec3(normal), view_dir, material, 6, point_light_count);
    out_color += CalcPointLight(point_lights[7], frag_pos, vec3(normal), view_dir, material, 7, point_light_count);
    out_color += CalcPointLight(point_lights[8], frag_pos, vec3(normal), view_dir, material, 8, point_light_count);
    out_color += CalcPointLight(point_lights[9], frag_pos, vec3(normal), view_dir, material, 9, point_light_count);
    out_color += CalcPointLight(point_lights[10], frag_pos, vec3(normal), view_dir, material, 10, point_light_count);
    out_color += CalcPointLight(point_lights[11], frag_pos, vec3(normal), view_dir, material, 11, point_light_count);
    out_color += CalcPointLight(point_lights[12], frag_pos, vec3(normal), view_dir, material, 12, point_light_count);
    out_color += CalcPointLight(point_lights[13], frag_pos, vec3(normal), view_dir, material, 13, point_light_count);
    out_color += CalcPointLight(point_lights[14], frag_pos, vec3(normal), view_dir, material, 14, point_light_count);
    out_color += CalcPointLight(point_lights[15], frag_pos, vec3(normal), view_dir, material, 15, point_light_count);
    return out_color;
}


/// -- shader code
in vec3 frag_pos;
in vec3 frag_normal;
in vec4 cam_pos;

in vec2 t_coords;
uniform sampler2D gradient;
uniform float damage;
uniform float time;
out vec4 out_color;

void main(void) {
    vec3 dmg_clr = vec3(1.0, 0.0, 0.0);
    float dmg = damage * abs(sin(time));
    vec3 diffuse_clr = mix(vec3(texture(gradient, t_coords)), dmg_clr, dmg);
    Material material = Material(
        diffuse_clr,
        diffuse_clr,
        diffuse_clr,
        16.0 
    );
    out_color = vec4(CalculateLights(frag_pos, vec3(frag_normal), vec3(cam_pos), material), 1.0);
} 

