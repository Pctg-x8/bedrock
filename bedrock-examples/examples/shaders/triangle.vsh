#version 450

layout(location = 0) in vec4 pos;
layout(location = 1) in vec4 col;
layout(location = 0) out vec4 ocol;
out gl_PerVertex { vec4 gl_Position; };

layout(push_constant) uniform ViewportParams {
    vec2 pixelSize;
};

layout(std140, set = 0, binding = 0) uniform ObjectParams {
    float rot;
};

void main() {
    mat4 r = mat4(cos(rot), -sin(rot), 0.0f, 0.0f, sin(rot), cos(rot), 0.0f, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f);
    gl_Position = (pos * r) / vec4(pixelSize * 0.5f, 1.0f, 1.0f);
    ocol = col;
}
