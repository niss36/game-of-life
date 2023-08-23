#version 300 es

uniform vec2 window_size;

in vec2 coordinates;
in vec2 position;

flat out vec2 varying_coordinates;

void main() {
    varying_coordinates = coordinates;

    gl_Position = vec4((position / window_size) * 2.f - vec2(1.f, 1.f), 0, 1);
}
