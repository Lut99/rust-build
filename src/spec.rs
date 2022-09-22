//  SPEC.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:01:47
//  Last edited:
//    21 Sep 2022, 18:19:24
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs that are used to interface
//!   with the framework. This generally include things that do not a lot
//!   of thinking themselves, but more provides the definitions or
//!   specification.
// 

use std::collections::HashSet;
use std::rc::Rc;

use crate::errors::BuildError;


/***** HELPER FUNCTIONS *****/
/// Builds the dependencies in one run, trying to avoid dependencies being build more than necessary.
/// 
/// # Arguments
/// - `target`: The Target who's dependencies to build.
fn build_target(target: &Rc<dyn Target>, built: HashSet<String>) -> Result<(), BuildError> {
    /* Check 1: Do any of the dependencies need rebuilding? */
    // Yes they do, or at least, we dumly assume so
    for d in target.deps() {
        // Allow the dependency to rebuild itself
        d.target.build()?;

        // Examine if any of the files we depend on have have changed
    }

    // Done
    Ok(())
}





/***** LIBRARY *****/
/// The basic trait that defines a build target.
pub trait Target: 'static {
    // Child-specific
    /// Builds the Target itself.
    /// 
    /// This function should do whatever it means to build the target itself. It may assume it will only be called if the build system has some reason for doing so.
    /// 
    /// # Returns
    /// Whether or not anything changed during the built - or more specifically, if any Targets depending on this dependency need to be updated too.
    fn build(&self) -> Result<(), BuildError>;
    /// Runs a check for any non-standard reasons for why a Target might need to be rebuild.
    /// 
    /// Any standard stuff, like the presence/up-to-dateness of certain files or directories is left 

    /// Returns the name of the Target.
    fn name(&self) -> &String;
    /// Returns an iterator over the dependencies for this Target.
    fn deps(&self) -> std::slice::Iter<Dependency>;



    // Global
    /// Builds the dependencies for this Target.
    /// 
    /// # Returns
    /// Whether or not any dependencies have been rebuilt.
    /// 
    /// # Errors
    /// This function errors if one of the dependencies failed to be built.
    fn build_deps(&self) -> Result<bool, BuildError> {
        /* TODO */
        Ok(false)
    }
}



/// Defines what we need to know of a dependency on a certain Target.
pub struct Dependency {
    /// The target on which we depend.
    pub target : Rc<dyn Target>,
}
