#version 330 core

flat in int vFaceID;
out vec4 FragColor;

void main()
{
    // couleur de base par face
    float base = fract(sin(float(vFaceID) * 12.9898) * 43758.5453);

    // petit dégradé interne basé sur la position écran
    float gradient = gl_FragCoord.x + gl_FragCoord.y;

    gradient = sin(gradient * 0.01);

    // mélange : couleur stable + variation locale
    float gray = mix(base, base * gradient, 0.3);

    FragColor = vec4(vec3(gray), 1.0);
}