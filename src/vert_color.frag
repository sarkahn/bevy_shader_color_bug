#version 450
layout(location = 0) in vec3 Vertex_Color;

layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = vec4(Vertex_Color, 1.0);
}