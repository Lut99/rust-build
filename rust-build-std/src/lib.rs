//  LIB.rs
//    by Lut99
// 
//  Created:
//    14 Nov 2022, 18:32:47
//  Last edited:
//    14 Nov 2022, 18:34:49
//  Auto updated?
//    Yes
// 
//  Description:
//!   The `rust-build-std` crate provides a few standard, often-used
//!   effects and targets for the `rust-build` crate.
// 

// Declare dependency/effect modules
pub mod effects;
pub use effects as deps;
pub mod targets;


// Define a few useful crate-local macros
/// A feature-dependent `trace` macro.
#[cfg(feature = "log")]
macro_rules! trace {
    ($($t:tt)*) => {
        log::trace!($($t)*)
    };
}
#[cfg(not(feature = "log"))]
macro_rules! trace {
    ($($t:tt)*) => {
        // Do not use them
    };
}
pub(crate) use trace;

/// A feature-dependent `warn` macro.
#[cfg(feature = "log")]
macro_rules! warning {
    ($($t:tt)*) => {
        log::warn!($($t)*)
    };
}
#[cfg(not(feature = "log"))]
macro_rules! warning {
    ($($t:tt)*) => {
        // Do not use them
    };
}
pub(crate) use warning as warn;
