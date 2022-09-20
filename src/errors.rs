//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:00:31
//  Last edited:
//    20 Sep 2022, 22:48:06
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines errors for the `rust-build` crate.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** LIBRARY *****/
/// The toplevel error of the crate.
#[derive(Debug)]
pub enum BuildError {
    
}

impl Display for BuildError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use BuildError::*;
        match self {
            
        }
    }
}

impl Error for BuildError {}
