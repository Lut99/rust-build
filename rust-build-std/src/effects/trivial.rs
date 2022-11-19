//  TRIVIAL.rs
//    by Lut99
// 
//  Created:
//    14 Nov 2022, 17:56:46
//  Last edited:
//    19 Nov 2022, 11:43:12
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines a few trivial dependencies and effects, which are essential
//!   for driving builds.
// 

use rust_build::spec::{Effect, Named};

use crate::trace;


/***** LIBRARY *****/
/// Defines an Effect that does nothing, but always returns it has been updated.
pub struct TrueEffect;

impl Named for TrueEffect {
    #[inline]
    fn name(&self) -> &str { "<true>" }
}
impl Effect for TrueEffect {
    #[inline]
    fn has_changed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        trace!("Marking '{}' as changed (always outdated)", self.name());
        Ok(true)
    }

    #[inline]
    fn commit_change(&self, _dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
        trace!("{}: Updating cache (virtually)", self.name());
        Ok(())
    }
}



/// Defines an Effect that does nothing, and always returns it hasn't been updated.
pub struct FalseEffect;

impl Named for FalseEffect {
    #[inline]
    fn name(&self) -> &str { "<false>" }
}
impl Effect for FalseEffect {
    #[inline]
    fn has_changed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        trace!("Marking '{}' as unchanged (always up-to-date)", self.name());
        Ok(false)
    }

    #[inline]
    fn commit_change(&self, _dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
        trace!("{}: Updating cache (virtually)", self.name());
        Ok(())
    }
}
