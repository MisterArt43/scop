PROJECT: SCOP (Mini-projet Infographie)
LANGUAGE: C (C99)
RENDERING: OpenGL (Core Profile)
WINDOWING: GLFW
CONSTRAINTS:
- No GLM
- No Assimp
- No stb_image
- No fixed pipeline (glBegin/glEnd)
- External libraries allowed ONLY for window/events + OpenGL loader (GLAD/GLEW)

==================================================
PHASE 0 — PROJECT SETUP
==================================================
[x] Define project directory structure:
    - src/
    - include/
    - shaders/
    - assets/obj/
    - assets/textures/
[x] Write Makefile (builds):
    - C compiler (cc)
    - flags: -Wall -Wextra -Werror -std=c99
    - link OpenGL + GLFW
[x] Verify clean build (no warnings)

==================================================
PHASE 1 — GLFW + OPENGL INITIALIZATION
==================================================
[ ] Initialize GLFW
[ ] Request OpenGL core profile context
[ ] Create window
[ ] Initialize OpenGL loader (GLAD or GLEW)
[ ] Enable depth testing (glEnable(GL_DEPTH_TEST))
[ ] Implement main render loop
[ ] Handle clean window close

GOAL: stable black window

==================================================
PHASE 2 — INPUT MANAGEMENT
==================================================
[ ] Define input state structure (keyboard)
[ ] Register GLFW key callback
[ ] Handle:
    - ESC → exit
    - Object translation (X/Y/Z + and -)
    - Texture toggle key
[ ] Maintain key pressed/released state

==================================================
PHASE 3 — SHADER SYSTEM
==================================================
[ ] Load vertex shader from file
[ ] Load fragment shader from file
[ ] Compile shaders
[ ] Link shader program
[ ] Implement shader error logging
[ ] Define uniforms:
    - mat4 MVP
    - bool texture_enabled
    - float texture_mix

==================================================
PHASE 4 — OPENGL BUFFERS
==================================================
[ ] Define vertex layout:
    - position (vec3)
    - color (vec3 or vec4)
    - UV (vec2)
[ ] Create VAO
[ ] Create VBO
[ ] Create EBO
[ ] Bind vertex attributes using glVertexAttribPointer

==================================================
PHASE 5 — MATH LIBRARY (CUSTOM)
==================================================
[ ] Define vec3 structure
[ ] Define vec4 structure
[ ] Define mat4 structure
[ ] Implement:
    - mat4_identity
    - mat4_translate
    - mat4_rotate_x
    - mat4_rotate_y
    - mat4_rotate_z
    - mat4_perspective
    - mat4_multiply
[ ] Upload matrices to shader via uniforms

==================================================
PHASE 6 — OBJ FILE PARSER (FROM SCRATCH)
==================================================
[ ] Read .obj file line by line
[ ] Parse:
    - v (vertex positions)
    - vt (texture coordinates)
    - f (faces)
[ ] Handle face formats:
    - v
    - v/vt
    - v/vt/vn (vn may be ignored initially)
[ ] Convert faces to triangles
[ ] Generate:
    - vertex buffer data
    - index buffer data
[ ] Compute object barycenter
[ ] Center object around origin

TARGET: display 42 logo correctly

==================================================
PHASE 7 — 3D RENDERING
==================================================
[ ] Setup perspective projection
[ ] Setup camera/view matrix
[ ] Apply model matrix
[ ] Compute MVP matrix
[ ] Render object using glDrawElements
[ ] Enable Z-buffer rendering
[ ] Apply per-face grayscale coloring

==================================================
PHASE 8 — OBJECT ANIMATION
==================================================
[ ] Implement deltaTime calculation
[ ] Rotate object around its own center
[ ] Ensure FPS-independent animation

==================================================
PHASE 9 — OBJECT MOVEMENT
==================================================
[ ] Translate object along:
    - X axis
    - Y axis
    - Z axis
[ ] Apply transformations through model matrix

==================================================
PHASE 10 — TEXTURE SYSTEM (CUSTOM LOADER)
==================================================
[ ] Implement basic texture loader (BMP or PPM)
[ ] Upload texture to OpenGL
[ ] Bind texture to sampler2D
[ ] Apply UV mapping
[ ] Toggle texture ON/OFF via input
[ ] Implement smooth transition using mix()

==================================================
PHASE 11 — FINAL SHADERS
==================================================
[ ] Vertex shader:
    - transform position with MVP
    - pass color and UV
[ ] Fragment shader:
    - display face color
    - sample texture
    - blend using texture_mix uniform

==================================================
PHASE 12 — FINAL VALIDATION
==================================================
[ ] 42 logo:
    - centered
    - rotates around its center
    - grayscale per face
    - texture applied with smooth transition
[ ] Test with multiple .obj files
[ ] No crashes
[ ] No warnings
[ ] Clean Git repository

==================================================
PHASE 13 — BONUS (ONLY IF ALL ABOVE IS PERFECT)
==================================================
[ ] Handle ambiguous / concave / non-coplanar .obj files
[ ] Improved UV mapping (no stretching)
[ ] Automatic object normalization
[ ] Anti-aliasing

END OF TASK LIST
