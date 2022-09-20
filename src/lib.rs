//  LIB.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 21:59:48
//  Last edited:
//    20 Sep 2022, 22:57:13
//  Auto updated?
//    Yes
// 
//  Description:
//!   The rust-build library provides a framework for writing
//!   object-oriented build scripts (a.k.a. installers due to the fact
//!   that it's Rust and they have to be compiled). The need for this
//!   arises due to some projects using Docker, which has bad integration
//!   with Makefiles and CMake and Cargo and whatnot.
//!   
//!   This library aims to
//!   provide an abstraction over Cargo to build Rust projects suitable
//!   for, among other things, use in Docker containers.
// 

// Declare modules
pub mod errors;
pub mod spec;
pub mod style;
pub mod targets;
pub mod installer;
#[cfg(test)]
pub mod tests;


// Pull some things into the global namespace
pub use errors::BuildError as Error;
pub use installer::Installer;
