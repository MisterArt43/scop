#version 330 core
out vec4 FragColor;

uniform vec3 uColor;
uniform vec3 uColorA;
uniform vec3 uColorB;
uniform int uUseGradient;
uniform int uGradientUseUV;
uniform float uMinY;
uniform float uMaxY;

in vec3 vWorldPos;
in vec2 vUV;

void main()
{
   if (uUseGradient == 0)
   {
      FragColor = vec4(uColor, 1.0f);
      return;
   }

   float t;
   if (uGradientUseUV != 0)
      t = vUV.y;
   else
      t = (vWorldPos.y - uMinY) / max(uMaxY - uMinY, 0.00001);

   t = clamp(t, 0.0, 1.0);
   vec3 c = mix(uColorA, uColorB, t);
   FragColor = vec4(c, 1.0f);
}