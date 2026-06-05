#version 330 core

in vec3 FragPos;
in vec3 Normal;
in vec3 vertexColor;
in vec2 TexCoord;

out vec4 FragColor;

uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 viewPos;

uniform vec3 materialDiffuse;   // Kd
uniform vec3 materialSpecular;  // Ks
uniform float shininess;        // Ns

uniform sampler2D diffuseTexture;     // Texture de diffuse
uniform bool hasTexture;              // Flag pour savoir si on a une texture
uniform bool debugUV;                 // Mode debug pour afficher les UV

void main()
{
    // Mode debug : affiche les coordonnées UV comme des couleurs (R=U, G=V)
    if (debugUV) {
        vec2 texCoordFlipped = vec2(TexCoord.x, 1.0 - TexCoord.y);
        texCoordFlipped = clamp(texCoordFlipped, 0.0, 1.0);
        FragColor = vec4(texCoordFlipped.x, texCoordFlipped.y, 0.0, 1.0);
        return;
    }

    vec3 N = normalize(Normal);
    vec3 L = normalize(lightPos - FragPos);

    // Lambert (diffus)
    float diff = max(dot(N, L), 0.0);
    
    // Si on a une texture, on l'utilise, sinon on utilise la couleur du matériau
    vec3 diffuseColor;
    if (hasTexture) {
        // Inverser l'axe Y des UV car OpenGL vs OBJ format différent
        vec2 texCoordFlipped = vec2(TexCoord.x, 1.0 - TexCoord.y);
        // Clamp les UV au cas où ils seraient hors de [0, 1]
        texCoordFlipped = clamp(texCoordFlipped, 0.0, 1.0);
        diffuseColor = texture(diffuseTexture, texCoordFlipped).rgb;
    } else {
        diffuseColor = materialDiffuse;
    }
    
    vec3 diffuse = diffuseColor * lightColor * diff;

    // Phong (spéculaire)
    vec3 V = normalize(viewPos - FragPos);
    vec3 R = reflect(-L, N);

    float spec = pow(max(dot(V, R), 0.0), shininess);
    vec3 specular = materialSpecular * lightColor * spec;

    // Couleur finale
    vec3 color = vertexColor * (diffuse + specular);

    FragColor = vec4(color, 1.0);
}