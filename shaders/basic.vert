#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aUV;

out vec3 vWorldPos;
out vec2 vUV;

uniform float scale;
uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

void main()
{
   vec3 scaledPos = aPos * (1.0 + scale);
   vec4 worldPos = uModel * vec4(scaledPos, 1.0);
   vWorldPos = worldPos.xyz;
   vUV = aUV;
   gl_Position = uProjection * uView * worldPos;
}