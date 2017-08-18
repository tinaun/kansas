#version 330

in vec2 a_Pos;
in vec2 a_UV;

uniform vec2 i_View;

out vec2 v_UV;

void main() {
    v_UV = a_UV * i_View;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}