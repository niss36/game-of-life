#version 300 es

uniform vec2 universe_size;

in uint cell;
in vec2 position;

flat out uint varying_cell;

void main() {
    varying_cell = cell;

    gl_Position = vec4((position / universe_size) * 2.f - vec2(1.f, 1.f), 0, 1);
}
