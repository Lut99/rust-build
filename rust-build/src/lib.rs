//  LIB.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 21:59:48
//  Last edited:
//    16 Nov 2022, 18:06:25
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
pub mod view;
pub mod cache;
pub mod style;
pub mod installer;
#[cfg(test)]
pub mod tests;


// Pull some things into the global namespace
pub use errors::BuildError as Error;
pub use installer::{Builder, Installer};


// Define some useful macros
/// A feature-dependent `debug` macro.
#[cfg(feature = "log")]
macro_rules! debug {
    ($($t:tt)*) => {
        log::debug!($($t)*)
    };
}
#[cfg(not(feature = "log"))]
macro_rules! debug {
    ($($t:tt)*) => {
        // Do not use them
    };
}
pub(crate) use debug;
