#version 330

in vec2 position;
in vec3 color;

out vec3 vColor;

uniform mat4 projectionMatrix;
uniform mat4 modelMatrix;

void main() {
    gl_Position = projectionMatrix * modelMatrix * vec4(position, 0.0, 1.0);
    vColor = color;
}
