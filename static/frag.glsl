#version 330

uniform sampler2D t_Canvas;

in vec2 v_UV;
out vec4 Target0;

void main() {
    vec4 canvas = texture(t_Canvas, v_UV).rgba;

    Target0 = canvas;
}

