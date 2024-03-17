
<img src="https://github.com/MichalMyczkowski/ruge/assets/63677127/99a8e4f7-85e1-4766-9197-73ad5bc0760e" width="210" height="60">

# Rust micro game engine

Ruge is a minimal framework which aims to provide a similiar user experience to Unity but in rust.
This project is in a very early stage and right now it is only possible to use it with OpenGL backend.
It came to life so I didn't have to use C++ during Computer Graphics course on my University.

## Features:
* Interchangable backends (right now only GLFW + OpenGL implemented)
* Scenes and GameObjects concepts
* Transform and Camera components
* Thin abstraction over OpenGL (program compilation, textures, etc.)
* Tested on Linux and MacOs


If you are interested, checkout the example projects:

```sh=

cargo run --example [EXAMPLE_NAME]

```

## Screenshots:

![rotating_cube](https://github.com/MichalMyczkowski/ruge/assets/63677127/50a30e14-b512-437e-b997-584e500843fb)
![3d_maze](https://github.com/MichalMyczkowski/ruge/assets/63677127/57f84fb5-9387-4aac-83b9-a73bb73e1a88)
![volcano](https://github.com/MichalMyczkowski/ruge/assets/63677127/3d44abe8-8bcd-4e7c-9df8-1cd33e2bd694)
