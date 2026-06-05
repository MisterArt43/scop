#version 330 core

layout(location = 0) in vec3 aPos;

flat out int vFaceID;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    vFaceID = gl_VertexID / 3;

    gl_Position = projection * view * model * vec4(aPos, 1.0);
}