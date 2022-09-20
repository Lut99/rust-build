# rust-build: A framework for writing installers
A repository that provides a framework for whipping up build scripts (installers) in Rust.

The need for this project arises mostly when building microservice architectures for use with Docker. I tend to gravitate towards projects that are split into multiple crates, some of which are binaries and some of which are just libraries, and building projects takes long (since its Rust).

Because it can get really nontrivial to build Rust for Docker containers with efficiency in both development and deloyment, this project culminates into an as-idiomatic-as-possible framework for dealing with this.


## Philosophy
This project approaches the problem of properly managing large builds in a "battery included" way.

The idea is that is makes it as easy as possible to make a custom installer executable for your project. This installer should be used to compile the source code from scratch (including making Docker containers), and then 'install' it appropriately (e.g., copy binaries around, load Docker images into the engine, ...).

This project should thus be thought of as a framework that will help you speed up creating such an installer. It is not a build system itself, like Makefile, that has to be pre-installed on your user's machine and uses stuff like custom domain-specific languages to configure it.


## Usage
To make installers of your own, you will typically see the following workflow.

First, you create your own project that has multiple containers. Having it set up as a workspace is a big pro, since that provides the smoothest interface with respect to compiling the installer.

Create a new crate in your workspace that is your installer (the name does not matter, but for this README we will use `installer`). Then, add this library as a dependency to that crate's `Cargo.toml`:
```toml
rust-build = "0.1.0"
```

You can then use the library to (hopefully) quickly and easily define your installer (to know how, check the examples in the `tests/` folder in this repository).

Once you have your installer crate programmed and ready, you should instruct your users to compile only that crate with Cargo:
```bash
cargo build --release --package installer
```
(where `installer` should be replaced by the name you chose for your installer crate).

With the installer compiled for their system, they can then run it according to how you configured it. For example:
```bash
./target/release/installer build all
```


## Contributing
If you want to contribute to this project, or have any questions or comments, feel free to drop an issue in the [issues](https://github.com/Lut99/rust-build/issues) page. Please tag your issue appropriately.

Please note that this project is mostly for myself; if you intend to use it for your own projects, please do! But remember that this software is provided "as-is" and that I'm not promising to help you or change things for you in any way (although I might if you ask nicely).
