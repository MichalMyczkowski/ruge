#version 330 
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

float calculate_specular(vec3 viewDir, vec3 normal, vec3 lightDir, float shininess) {
     // Calculate the cosine of the angle between the reflection vector
     // and the vector going to the camera.
     vec3 reflection = normalize(2.0 * dot(normal, lightDir) * normal - lightDir);
     viewDir = normalize( -viewDir );
     float cos_angle = dot(reflection, -viewDir);
     cos_angle = clamp(cos_angle, 0.0, 1.0);
     return pow(cos_angle, shininess);
}

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir, Material material)
{
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


vec3 CalculateLights(vec3 frag_pos, vec3 normal, vec3 camera_pos, Material material) {
    vec3 view_dir = normalize(frag_pos - vec3(camera_pos));

    LightColor light_color = LightColor(
        vec4(1.0, 1.0, 1.0, 1.0),
        vec4(1.0, 1.0, 1.0, 1.0),
        vec4(1.0, 1.0, 1.0, 1.0)
    );
    DirLight dir_light = DirLight (
        vec4(-1.0, -1.0, -1.0, 0.0),
        light_color

    );
    return CalcDirLight(dir_light, vec3(normal), view_dir, material);
}

in vec3 frag_pos;
in vec3 frag_normal;
in vec4 cam_pos;

out vec4 out_color;

void main(void) {
    vec3 diffuse_clr = vec3(0.8, 0.7, 0.5);
    Material material = Material(
        diffuse_clr * 0.2,
        diffuse_clr,
        vec3(1.0, 1.0, 1.0),
        16.0 
    );
    out_color = vec4(CalculateLights(frag_pos, vec3(frag_normal), vec3(cam_pos), material), 1.0);
} 
