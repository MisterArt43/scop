#version 330 core

in vec3 vertexColor;

out vec4 FragColor;

uniform vec3 materialDiffuse;

void main()
{
    FragColor = vec4(vertexColor * materialDiffuse, 1.0);
}