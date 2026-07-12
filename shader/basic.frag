#version 330 core

// Entrées en provenance du Vertex Shader
in vec3 FragPos;
in vec3 Normal;
in vec3 vertexColor;
in vec2 TexCoord;

// Sortie unique vers le framebuffer
out vec4 FragColor;

// Uniforms de scène et lumières
uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 viewPos;

// Uniforms de matériaux (Calqués sur ton code Rust)
uniform vec3 materialDiffuse;     // Kd
uniform vec3 materialSpecular;    // Ks
uniform float materialShininess;  // Ns
uniform vec3 materialAmbient;     // Ka
uniform float materialAlpha;      // d / Tr

// Modèle d'illumination (0, 1 ou 2)
uniform int illuminationModel; 

// Textures et commutateurs
uniform sampler2D diffuseTexture;
uniform sampler2D specularTexture;
uniform bool hasDiffuse;
uniform bool hasSpecular;

void main()
{
    // --- 1. Échantillonnage Diffuse & Alpha ---
    vec3 diffuseColor = materialDiffuse;
    float alpha = materialAlpha;

    if (hasDiffuse) {
        // Retournement optionnel de l'axe Y si tes textures sont à l'envers
        vec2 texCoordFlipped = vec2(TexCoord.x, 1.0 - TexCoord.y);
        texCoordFlipped = clamp(texCoordFlipped, 0.0, 1.0);
        
        vec4 texColor = texture(diffuseTexture, texCoordFlipped);
        diffuseColor = texColor.rgb;
        alpha *= texColor.a; // Cumul de l'alpha du matériau et de l'image
    }

    // --- 2. Calcul des modèles d'illumination ---
    vec3 finalColor = vec3(0.0);

    if (illuminationModel == 0) {
        // Mode 0 : Pas de calcul de lumière (Unlit / Couleur pure)
        finalColor = diffuseColor;
    } 
    else {
        // Préparation des vecteurs nécessaires pour la lumière
        vec3 N = normalize(Normal);
        vec3 L = normalize(lightPos - FragPos);
        
        // Composante Ambiante
        vec3 ambient = materialAmbient * lightColor;
        
        // Composante Diffuse (Lambert)
        float diff = max(dot(N, L), 0.0);
        vec3 diffuse = diffuseColor * lightColor * diff;

        if (illuminationModel == 1) {
            // Mode 1 : Ambiant + Diffus (Pas de reflet brillant)
            finalColor = ambient + diffuse;
        } 
        else if (illuminationModel == 2) {
            // Mode 2 : Phong standard (Ambiant + Diffus + Spéculaire)
            vec3 V = normalize(viewPos - FragPos);
            vec3 R = reflect(-L, N);
            
            // Gestion de la couleur spéculaire de base (Texture vs Uniform)
            vec3 specColor = materialSpecular;
            if (hasSpecular) {
                vec2 texCoordFlipped = vec2(TexCoord.x, 1.0 - TexCoord.y);
                texCoordFlipped = clamp(texCoordFlipped, 0.0, 1.0);
                specColor = texture(specularTexture, texCoordFlipped).rgb;
            }

            float specFactor = pow(max(dot(V, R), 0.0), materialShininess);
            vec3 specular = specColor * lightColor * specFactor;
            
            finalColor = ambient + diffuse + specular;
        }
    }

    // --- 3. Teinte par sommet & Sortie ---
    // Si tes sommets n'ont pas de couleur (ou sont noirs par défaut), 
    // assure-toi que ton vertex shader passe vec3(1.0) dans vertexColor.
    finalColor *= vertexColor;

    FragColor = vec4(finalColor, alpha);
}