//  CARGO.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:03:29
//  Last edited:
//    21 Sep 2022, 18:11:37
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines a target that builds something using Cargo.
// 

use crate::errors::BuildError;
use crate::spec::{Dependency, Target};


/***** LIBRARY *****/
/// The Cargo target is used to build some Rust thing using Cargo. That also handles dependencies and junk.
pub struct CargoTarget {
    /// The name of the Target.
    name : String,
    /// The dependencies that must be built first before this Target is built.
    deps : Vec<Dependency>,

    /// The name of the package that will be built.
    package : String,
}

impl CargoTarget {
    /// Constructor for the CargoTarget that initializes it with for the given package.
    /// 
    /// # Arguments
    /// - `name`: The name of this Target.
    /// - `package`: The package that should be built with this target.
    /// - `deps`: The dependencies that must be built first before this target can be built.
    /// 
    /// # Returns
    /// A new instance of a CargoTarget.
    #[inline]
    pub fn new(name: impl Into<String>, package: impl Into<String>, deps: Vec<Dependency>) -> Self {
        Self {
            name : name.into(),
            deps,

            package : package.into(),
        }
    }
}

impl Target for CargoTarget {
    fn build(&self) -> Result<bool, BuildError> {
        Ok(false)
    }



    fn name(&self) -> &String { &self.name }

    fn deps(&self) -> std::slice::Iter<Dependency> { self.deps.iter() }
}
