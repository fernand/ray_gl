#version 330 core

in vec3 vertexColor;

out vec4 Color;

void main()
{
    Color = vec4(vertexColor, 1.0f);
}
