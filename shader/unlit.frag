#version 330 core

in vec2 TexCoord;
in vec3 vertexColor;

out vec4 FragColor;

uniform sampler2D diffuseTexture;
uniform bool hasTexture;

void main()
{
    if (hasTexture) {
        vec2 texCoordFlipped = vec2(TexCoord.x, 1.0 - TexCoord.y);
        texCoordFlipped = clamp(texCoordFlipped, 0.0, 1.0);
        FragColor = texture(diffuseTexture, texCoordFlipped);
    } else {
        FragColor = vec4(vertexColor, 1.0);
    }
}
