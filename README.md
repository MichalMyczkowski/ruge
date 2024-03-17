
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
