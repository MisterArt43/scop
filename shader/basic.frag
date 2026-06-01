#version 330 core

in vec3 FragPos;
in vec3 Normal;
in vec3 vertexColor;

out vec4 FragColor;

uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 viewPos;

uniform vec3 materialDiffuse;   // Kd
uniform vec3 materialSpecular;  // Ks
uniform float shininess;        // Ns

void main()
{
    vec3 N = normalize(Normal);
    vec3 L = normalize(lightPos - FragPos);

    // Lambert (diffus)
    float diff = max(dot(N, L), 0.0);
    vec3 diffuse = materialDiffuse * lightColor * diff;

    // Phong (spéculaire)
    vec3 V = normalize(viewPos - FragPos);
    vec3 R = reflect(-L, N);

    float spec = pow(max(dot(V, R), 0.0), shininess);
    vec3 specular = materialSpecular * lightColor * spec;

    // Couleur finale
    vec3 color = vertexColor * (diffuse + specular);

    FragColor = vec4(color, 1.0);
}