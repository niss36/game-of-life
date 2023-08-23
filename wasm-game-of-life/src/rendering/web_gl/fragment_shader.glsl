#version 300 es

precision highp float;

flat in uint varying_cell;

out vec4 outColor;

void main() {
    if(bool(varying_cell)) {
        outColor = vec4(0, 0, 0, 1);
    } else {
        outColor = vec4(1, 1, 1, 1);
    }
}
