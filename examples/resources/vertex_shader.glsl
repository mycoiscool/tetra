#version 330 core

in vec2 position;

out vec2 v_uv;

void main()
{
    v_uv = position * 0.5 + 0.5; // Transform from [-1,1] to [0,1]
    gl_Position = vec4(position, 0.0, 1.0);
}