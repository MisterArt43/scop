#version 330 core
out vec4 FragColor;

// shadertoys uniforms // pour la compatibilité

uniform vec3      iResolution;           // viewport resolution (in pixels)
uniform float     iTime;                 // shader playback time (in seconds)
uniform float     iTimeDelta;            // render time (in seconds)
uniform float     iFrameRate;            // shader frame rate
uniform int       iFrame;                // shader playback frame
uniform float     iChannelTime[4];       // channel playback time (in seconds)
uniform vec3      iChannelResolution[4]; // channel resolution (in pixels)
uniform vec4      iMouse;                // mouse pixel coords. xy: current (if MLB down), zw: click
// uniform samplerXX iChannel0..3;          // input channel. XX = 2D/Cube // PAS NECESSAIRE
uniform vec4      iDate;                 // (year, month, day, time in seconds)

// mes uniforms

uniform vec3 uColor;
uniform vec3 uColorA;
uniform vec3 uColorB;
uniform int uUseGradient;
uniform int uGradientUseUV;
uniform int uUseTexture;
uniform sampler2D uTexture;
uniform int uUvMode;
uniform vec2 uUvScale;
uniform vec2 uUvOffset;
uniform float uMinY;
uniform float uMaxY;

in vec3 vWorldPos;
in vec2 vUV;

void main()
{
   if (uUseTexture != 0)
   {
     vec2 uv = vUV;
     // uUvMode:
     // 0=none, 1=flipU, 2=flipV, 3=flipUV, 4=swap, 5=swap+flipU, 6=swap+flipV, 7=swap+flipUV
     if (uUvMode >= 4)
        uv = uv.yx;
     if (uUvMode == 1 || uUvMode == 3 || uUvMode == 5 || uUvMode == 7)
        uv.x = 1.0 - uv.x;
     if (uUvMode == 2 || uUvMode == 3 || uUvMode == 6 || uUvMode == 7)
        uv.y = 1.0 - uv.y;
     uv = uv * uUvScale + uUvOffset;
      //DEBUG au dessus

      FragColor = vec4(texture(uTexture, uv).rgb, 1.0f);
      return;
   }

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