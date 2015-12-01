# Progress Report

For our final project, we are experimenting with OpenGL shaders and writing some demos. We have chosen to use `glium`, a safe OpenGL library for the Rust programming language.

### Tools

[Rust](https://www.rust-lang.org) is a safe, functional language intended for systems programming. It is intended to play a similar role to C++, but incorporating ideas from functional programming languages such as Haskell and ML. A major feature of Rust is its compile-time memory management, permitting memory safety to be guaranteed at the language level without the use of garbage collection. [`glium`](https://github.com/tomaka/glium) is a Rust library that provides high-level bindings into OpenGL.

### Our Progress

Thus far we have spent a great deal of our time learning more about graphics programming in OpenGL. We've been reading through the introductory chapters of _The OpenGL SuperBible_ and studying OpenGL syntax. We've also been looking at shaders on [shadertoy.com](https://www.shadertoy.com), a website which uses WebGL to run OpenGL shader demos in the browser, and reading over & trying to understand some of their source code.

Furthermore, we've started working on writing programs for our project. We've configured a basic project using `cargo`, Rust's build tool, and added `glium` as a dependency. Our project goes through the basic steps of creating a `glium` window, and compiles some very simple shaders to create a simple animation of a spinning triangle. Now that most of the "plumbing" is out of the way, we can start writing some fancier shaders.

Our code doesn't currently work on Alden lab computers, since the Rust compiler requires root access to install. However, I'm working on configuring Travis CI to continually build our code, so there will always be an Ubuntu binary available to download and run.
