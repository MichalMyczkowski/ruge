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
uniform float time;

out vec2 texture_coord;
out vec4 color;

// Fast 3D perlin noise implementation from https://www.shadertoy.com/view/slB3z3
// implementation of MurmurHash (https://sites.google.com/site/murmurhash/) for a 
// single unsigned integer.

uint hash(uint x, uint seed) {
    const uint m = 0x5bd1e995U;
    uint hash = seed;
    // process input
    uint k = x;
    k *= m;
    k ^= k >> 24;
    k *= m;
    hash *= m;
    hash ^= k;
    // some final mixing
    hash ^= hash >> 13;
    hash *= m;
    hash ^= hash >> 15;
    return hash;
}

// implementation of MurmurHash (https://sites.google.com/site/murmurhash/) for a  
// 3-dimensional unsigned integer input vector.

uint hash(uvec3 x, uint seed){
    const uint m = 0x5bd1e995U;
    uint hash = seed;
    // process first vector element
    uint k = x.x; 
    k *= m;
    k ^= k >> 24;
    k *= m;
    hash *= m;
    hash ^= k;
    // process second vector element
    k = x.y; 
    k *= m;
    k ^= k >> 24;
    k *= m;
    hash *= m;
    hash ^= k;
    // process third vector element
    k = x.z; 
    k *= m;
    k ^= k >> 24;
    k *= m;
    hash *= m;
    hash ^= k;
	// some final mixing
    hash ^= hash >> 13;
    hash *= m;
    hash ^= hash >> 15;
    return hash;
}


vec3 gradientDirection(uint hash) {
    switch (int(hash) & 15) { // look at the last four bits to pick a gradient direction
    case 0:
        return vec3(1, 1, 0);
    case 1:
        return vec3(-1, 1, 0);
    case 2:
        return vec3(1, -1, 0);
    case 3:
        return vec3(-1, -1, 0);
    case 4:
        return vec3(1, 0, 1);
    case 5:
        return vec3(-1, 0, 1);
    case 6:
        return vec3(1, 0, -1);
    case 7:
        return vec3(-1, 0, -1);
    case 8:
        return vec3(0, 1, 1);
    case 9:
        return vec3(0, -1, 1);
    case 10:
        return vec3(0, 1, -1);
    case 11:
        return vec3(0, -1, -1);
    case 12:
        return vec3(1, 1, 0);
    case 13:
        return vec3(-1, 1, 0);
    case 14:
        return vec3(0, -1, 1);
    case 15:
        return vec3(0, -1, -1);
    }
}

float interpolate(float value1, float value2, float value3, float value4, float value5, float value6, float value7, float value8, vec3 t) {
    return mix(
        mix(mix(value1, value2, t.x), mix(value3, value4, t.x), t.y),
        mix(mix(value5, value6, t.x), mix(value7, value8, t.x), t.y),
        t.z
    );
}

vec3 fade(vec3 t) {
    // 6t^5 - 15t^4 + 10t^3
	return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

float perlinNoise(vec3 position, uint seed) {
    vec3 floorPosition = floor(position);
    vec3 fractPosition = position - floorPosition;
    uvec3 cellCoordinates = uvec3(floorPosition);
    float value1 = dot(gradientDirection(hash(cellCoordinates, seed)), fractPosition);
    float value2 = dot(gradientDirection(hash((cellCoordinates + uvec3(1, 0, 0)), seed)), fractPosition - vec3(1, 0, 0));
    float value3 = dot(gradientDirection(hash((cellCoordinates + uvec3(0, 1, 0)), seed)), fractPosition - vec3(0, 1, 0));
    float value4 = dot(gradientDirection(hash((cellCoordinates + uvec3(1, 1, 0)), seed)), fractPosition - vec3(1, 1, 0));
    float value5 = dot(gradientDirection(hash((cellCoordinates + uvec3(0, 0, 1)), seed)), fractPosition - vec3(0, 0, 1));
    float value6 = dot(gradientDirection(hash((cellCoordinates + uvec3(1, 0, 1)), seed)), fractPosition - vec3(1, 0, 1));
    float value7 = dot(gradientDirection(hash((cellCoordinates + uvec3(0, 1, 1)), seed)), fractPosition - vec3(0, 1, 1));
    float value8 = dot(gradientDirection(hash((cellCoordinates + uvec3(1, 1, 1)), seed)), fractPosition - vec3(1, 1, 1));
    return interpolate(value1, value2, value3, value4, value5, value6, value7, value8, fade(fractPosition));
}

void main(void) {
    mat4 model = mat4(m_1, m_2, m_3, m_4);
    gl_Position = projection * model * vec4(vert, 1.0);
    int squared = size * size;
    int x = gl_InstanceID % size;
    int z = ((gl_InstanceID % squared) / size);
    int y = gl_InstanceID / squared;
    float xx = float(x)/float(size);
    float zz = float(z)/float(size);
    float yy = float(y)/float(size);
    // set color
    vec3 position = vec3(xx, yy, zz);
    float value = perlinNoise(vec3(position.xy, position.z + time), uint(12));
    value = clamp(value, 0.1, 1.0) * 2.0;
    // out values
    color = vec4(xx, zz, yy, 1.0) * value;
    texture_coord = tex_coord;

    //if (gl_InstanceID == last_instance || gl_InstanceID == 0) {
    //    color.a = 0.0;
    //}

}
