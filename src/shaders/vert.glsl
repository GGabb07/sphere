#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 norm;

uniform mat4 proj;

out vec3 fNorm;
out vec3 fPos;

void main() {
    fNorm = norm;
    vec4 temp = proj * vec4(pos, 1.);
    gl_Position = temp;
    fPos = temp.xyz;
}
