//  SPEC.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:01:47
//  Last edited:
//    13 Nov 2022, 16:46:22
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs that are used to interface
//!   with the framework. This generally include things that do not a lot
//!   of thinking themselves, but more provides the definitions or
//!   specification.
// 

use std::collections::binary_heap::Iter;
use std::error::Error;
use std::fmt::Debug;

use crate::errors::TargetError;
use crate::view::{EffectView, ViewFilter};


/***** LIBRARY *****/
/// Defines a named Dependency, Effect or Target.
pub trait Named {
    // Child-provided
    /// Returns the identifier of this Effect.
    fn name(&self) -> &str;
}



/// Defines a Depedency, which is some kind of object that has to perform some action before a subsequent Target can be run.
pub trait Dependency: Named {
    // Child-provided
    /// Determines if the depedency has been updated since the last time.
    /// 
    /// Typically, it makes sense to use the Cache for this.
    /// 
    /// # Returns
    /// 'true' if the dependency was updated (and thus warrants compilation by depending targets) or 'false' if it was not (and depending targets can thus assume this dependency to be unchanged).
    /// 
    /// # Errors
    /// This function may error for its own reasons.
    fn has_changed(&mut self) -> Result<bool, Box<dyn Error>>;
}



/// Defines an Effect, which is something that a Target produces. Typically (though not always), an Effect is also a Dependency such that future target may use it themselves.
pub trait Effect: Named {
    /// Updates the underlying mechanisms to "commit" the current state of the dependency as the 'last' state.
    /// 
    /// In practise, this typically means stuff like writing the last edited time of a file to the cache, for example.
    /// 
    /// # Errors
    /// If we failed  to update the underlying mechanisms, this function may throw an error.
    fn commit_change(&mut self) -> Result<(), Box<dyn Error>>;
}



/// Defines a Target, which is something that compiles, installs or runs something else.
pub trait Target: Named {
    // Globally available
    /// Builds any dependencies that this Target has defined. After this operation, it will be safe to call `Target::build()`.
    /// 
    /// Uses the `Target::deps()` function to determine those.
    /// 
    /// # Errors
    /// This function errors if we failed to build any of the targets this target depends on.
    fn build_deps(&self) -> Result<(), TargetError> {
        // Iterate over all of the views
        for view in self.deps() {
            // Build the target behind this view first.
            
        }

        // Done, everything is built
        Ok(())
    }



    // Child-provided
    /// Builds this Target as it likes.
    /// 
    /// You can assume that this function is only called if the dependencies have been build _and_ produced any changes in the effects that we depend upon.
    /// 
    /// After this operation, it will be safe to call `Target::commit()`.
    /// 
    /// # Errors
    /// This function errors if we failed to build this target.
    fn build(&self) -> Result<(), TargetError> {
        
    }



    /// Returns a TargetView on this Target's effects.
    /// 
    /// This can be used to not depend on all of its effects, but rather a subset of them.
    /// 
    /// # Returns
    /// A new TargetView instance that can be used to describe the subset to depend on.
    #[inline]
    fn view<'a>(&'a self) -> EffectView<'a>
    where
        Self: Sized,
    {
        EffectView{
            target  : self,
            filters : vec![ ViewFilter::All ],
        }
    }
    /// Returns a TargetView on this Target's effects.
    /// 
    /// This can be used to not depend on all of its effects, but rather a subset consisting of the giving names only.
    /// 
    /// # Arguments
    /// - `names`: The Effect names to only depend on.
    /// 
    /// # Returns
    /// A new TargetView instance that can be used to describe the subset to depend on.
    #[inline]
    fn view_names<'a, 'b>(&'a self, names: impl Into<Vec<String>>) -> EffectView<'a>
    where
        Self: Sized,
    {
        EffectView{
            target  : self,
            filters : vec![ ViewFilter::Allow{ names: names.into() } ],
        }
    }



    // Child-provided
    /// Returns a list of dependencies of this Target. The ordering of them is irrelevant.
    /// 
    /// Note that they are as EffectViews instead of simple Dependencies to allow the target to only depend on a subset of a dependency.
    fn deps(&self) -> &[EffectView];
    /// Returns a list of effects that this Target produces. The ordering of them is irrelevant.
    fn effects(&self) -> &[Box<dyn Effect>];
}
