# Scop

This project has been created as part of the 42 curriculum by abucia

Scop est un visualiseur 3D ecrit en Rust avec OpenGL. Il charge un fichier OBJ, lit ses materiaux MTL, cree les buffers GPU necessaires au rendu, puis affiche le modele dans une fenetre GLFW.

Le binaire genere par le projet s'appelle `scop`.

## Compilation

Le projet utilise Cargo, mais un Makefile est fourni pour les commandes attendues.

```sh
make
```

La regle par defaut construit le binaire et copie l'executable a la racine du projet sous le nom `scop`.

Commandes utiles:

```sh
make all      # compile scop avec la regle par defaut
make debug    # compile scop en debug
make release  # compile scop en release
make run      # compile puis lance le visualiseur avec les valeurs par defaut
make check    # verifie le projet sans produire de binaire final
make clean    # nettoie les artefacts Cargo
make fclean   # nettoie les artefacts Cargo et supprime ./scop
make re       # reconstruit depuis zero
```

## Execution

Sans argument, le programme utilise les shaders et le modele par defaut:

```sh
./scop
```

Il est aussi possible de fournir un vertex shader, un fragment shader et un fichier OBJ:

```sh
./scop ./shader/solid.vert ./shader/solid.frag ./ressources/42.obj
```

## Controles

Pendant l'execution:

- `W`, `A`, `S`, `D`: deplacer la camera horizontalement
- `Q`, `E`: descendre ou monter la camera
- Fleches directionnelles: faire tourner la camera
- `+` et `-` du pave numerique: zoomer ou dezoomer
- `C`: alterner entre camera libre et camera orbitale
- `Alt gauche`: afficher temporairement le maillage en fil de fer
- `Shift gauche`: desactiver temporairement le culling
- `Echap`: fermer la fenetre

## Structure du code

Le point d'entree est `src/main.rs`. Il lit les arguments, initialise l'application, compile les shaders, charge le modele OBJ, envoie les textures a la GPU, puis execute la boucle de rendu.

Les principaux modules sont:

- `src/application.rs`: cree la fenetre GLFW, initialise OpenGL, gere les evenements clavier et souris.
- `src/camera.rs`: gere la camera libre, la camera orbitale, la projection et la matrice de vue.
- `src/mesh/`: parse les fichiers OBJ, construit les sous-maillages et cree les VAO, VBO et EBO.
- `src/material.rs`: regroupe les materiaux, le parsing MTL, les couleurs, les textures diffuse/speculaire et la resolution des chemins de textures.
- `src/texture.rs`: charge les textures supportees puis les envoie a OpenGL.
- `src/bmp.rs`: decode les images BMP utilisees comme textures.
- `src/shader.rs`: lit, compile et link les shaders GLSL, puis expose des helpers pour les uniforms.
- `src/math/`: contient les types mathematiques maison: vecteurs, matrices, quaternions et transforms.
- `src/vao.rs`, `src/vbo.rs`, `src/ebo.rs`: encapsulent les objets OpenGL de base.

## Chargement des modeles

Le parser OBJ lit:

- les positions `v`
- les normales `vn`
- les coordonnees UV `vt`
- les faces `f`
- les objets et groupes `o` / `g`
- les bibliotheques de materiaux `mtllib`
- les materiaux actifs `usemtl`

Quand un fichier MTL est reference, le parser charge les couleurs `Ka`, `Kd`, `Ks`, la brillance `Ns`, l'alpha, l'indice de refraction, le modele d'illumination et les textures diffuse/speculaire quand elles sont presentes.

## Shaders

Les shaders fournis sont dans `shader/`. Le shader par defaut est:

```sh
./shader/solid.vert
./shader/solid.frag
```

Le programme renseigne notamment les uniforms de matrices (`model`, `view`, `projection`), de lumiere (`lightPos`, `viewPos`, `lightColor`) et de materiau (`materialDiffuse`, `materialSpecular`, `materialShininess`, `diffuseTexture`, `hasTexture`).

## Ressources

Les modeles et textures de test sont dans `ressources/`. Le modele charge par defaut est:

```sh
./ressources/42.obj
```

Les textures actuellement prises en charge par le code sont les fichiers BMP. Le chemin des textures est resolu relativement au fichier OBJ/MTL, avec une recherche par nom quand le chemin exact ne correspond pas.

## IA

j'ai beaucoup utilisé en autocompletion avec qwen 2.5-coder 7B fine tune pour le rust via continue.dev (auto hebergé), 
la pluspart de mes recherche ont été faites sur le site de opengl directement (certain liens sont même référencé dans le code) et [learnopengl.com](https://learnopengl.com), enfin une partie du parsing des texture a été refacto par IA