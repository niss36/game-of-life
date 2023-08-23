#version 300 es

precision mediump float;

uniform vec2 universe_size;
uniform sampler2D cells;

flat in vec2 varying_coordinates;

out vec4 outColor;

void main() {
    vec2 texture_coordinates = varying_coordinates / universe_size;

    vec4 cell_value = texture(cells, texture_coordinates);

    if(cell_value.a > 0.f) {
        outColor = vec4(0, 0, 0, 1);
    } else {
        outColor = vec4(1, 1, 1, 1);
    }
}
