#version 450
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform CustomMaterial_color {
    vec4 color;
};
void main() {
    o_Target = color;
}