//  CARGO.rs
//    by Lut99
// 
//  Created:
//    13 Nov 2022, 14:34:33
//  Last edited:
//    13 Nov 2022, 15:23:08
//  Auto updated?
//    Yes
// 
//  Description:
//!   Provides a target for compiling Rust with some default options.
//! 
//!   Note that this Target uses the `File` dependency/effect, also
//!   provided in the standard library.
// 

use crate::spec::{Dependency, Effect, Target};


/***** LIBRARY *****/
/// Defines the Cargo target, which uses the Cargo build system to compile Rust code.
/// 
/// This can typically be used as a starting point in your dependency tree.
pub struct CargoTarget {
    /// The dependencies of this target.
    deps    : Vec<Box<dyn Dependency>>,
    /// The effects (that we care about) of this target.
    effects : Vec<Box<dyn Effect>>,
}

// impl Target for CargoTarget {
    
// }
