//  MOD.rs
//    by Lut99
// 
//  Created:
//    14 Nov 2022, 17:59:20
//  Last edited:
//    14 Nov 2022, 17:59:45
//  Auto updated?
//    Yes
// 
//  Description:
//!   The `library::effects` module (also known as `library::deps`)
//!   defines default (but customizable) dependencies/effects.
// 

// Declare the effects
pub mod trivial;
pub mod file;

// Pull some stuff into this module's namespace
pub use file::File;
