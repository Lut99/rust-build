# Hello world example
This workspace implements a very simple `Hello, world!` app (in fact, it defines Cargo's default one) and how to write an installer for that.


## The application
The application we will be installing is extremely simple. It will just be a binary ([`hello-world/src/main.rs`](./hello-world/src/main.rs)) that should be installed under a `bin/hello_world` directory.

The relevant source files are:
- [`hello-world/src/main.rs`](./hello-world/src/main.rs): The implementation of the `Hello, world!` app that we will install.
- [`hello-world-installer/src/main.rs`](./hello-world-installer/src/main.rs): The entrypoint to the installer that we've created.


## The installer

