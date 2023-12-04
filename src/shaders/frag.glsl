#version 330 core

in vec3 fNorm;
in vec3 fPos;

out vec4 fColor;

void main() {
    float dot = dot(fNorm, vec3(1., 0.75, 0.5));
    float diff = max(dot * dot * dot + .5, 0.25);
    fColor = vec4(0., diff, 0., 1.);
}