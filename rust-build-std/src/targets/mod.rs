//  MOD.rs
//    by Lut99
// 
//  Created:
//    14 Nov 2022, 17:58:22
//  Last edited:
//    14 Nov 2022, 17:58:41
//  Auto updated?
//    Yes
// 
//  Description:
//!   The `library::targets` module defines default (but customizable)
//!   targets.
// 

// Declare our targets
pub mod cargo;

// Pull stuff into this namespace
pub use cargo::CargoTarget;
