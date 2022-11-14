//  SPEC.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:01:47
//  Last edited:
//    14 Nov 2022, 18:03:10
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs that are used to interface
//!   with the framework. This generally include things that do not a lot
//!   of thinking themselves, but more provides the definitions or
//!   specification.
// 

use std::error::Error;

use crate::errors::TargetError;
use crate::view::{EffectView, ViewFilter};


/***** LIBRARY *****/
/// Defines a named Dependency, Effect or Target.
pub trait Named {
    // Child-provided
    /// Returns the identifier of this Effect.
    fn name(&self) -> &str;
}



/// Defines an Effect, which is something that a Target produces. Typically (though not always), an Effect is also a Dependency such that future target may use it themselves.
pub trait Effect: Named {
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
    fn has_changed(&self) -> Result<bool, Box<dyn Error>>;

    /// Updates the underlying mechanisms to "commit" the current state of the dependency as the 'last' state.
    /// 
    /// In practise, this typically means stuff like writing the last edited time of a file to the cache, for example.
    /// 
    /// Note that it's important that, if this function is _not_ called, no change is updated; or, on other words, the exact same files should be build in between runs if no `Effect::commit_change()` has been called.
    /// 
    /// # Arguments
    /// - `dry_run`: If 'true', prints what would be done instead of actually doing it.
    /// 
    /// # Errors
    /// If we failed  to update the underlying mechanisms, this function may throw an error. Note, however, that the change must also be uncommitted if this function errors.
    fn commit_change(&self, dry_run: bool) -> Result<(), Box<dyn Error>>;
}



/// Defines a Target, which is something that compiles, installs or runs something else.
pub trait Target: Named {
    // Globally available
    /// Builds the target's dependencies, itself and then commits the results to cache.
    /// 
    /// It's a shortcut for running `Target::build_deps()`, `Target::build()` and `Target::commit()` in succession.
    /// 
    /// # Arguments
    /// - `force`: If 'true', always builds all targets instead of only when there is no (detected) change.
    /// - `dry_run`: If 'true', prints what would be done instead of actually executing the commands. Note that this is an imperfect simulation, since effect changes cannot be accurately detected without actually changing them.
    /// 
    /// # Errors
    /// This function errors if any of the three other functions would error.
    fn make(&self, force: bool, dry_run: bool) -> Result<(), TargetError> {
        // Call the dependencies first, to find out if anything has to happen.
        let outdated: bool = self.build_deps(force, dry_run)?;

        // Next, if it does, run the build & commit
        if outdated {
            self.build(dry_run)?;
            self.commit(dry_run)?;
        }

        // Done
        Ok(())
    }

    /// Builds any dependencies that this Target has defined. After this operation, it will be safe to call `Target::build()`.
    /// 
    /// Uses the `Target::deps()` function to determine those.
    /// 
    /// # Arguments
    /// - `force`: If 'true', always builds all dependencies instead of only when there is no (detected) change to their dependencies.
    /// - `dry_run`: If 'true', prints what would be done instead of actually executing the commands. Note that this is an imperfect simulation, since effect changes cannot be accurately detected without actually changing them.
    /// 
    /// # Returns
    /// Whether any of the resulting cache files is outdated or not, and thus whether this Target should be rebuild or not. If `force` is true, then this also always returns true.
    /// 
    /// # Errors
    /// This function errors if we failed to build any of the targets this target depends on.
    fn build_deps(&self, force: bool, dry_run: bool) -> Result<bool, TargetError> {
        // Iterate over all of the views
        let mut outdated: bool = force;
        for view in self.deps() {
            // Build the target behind this view first.
            view.target.make(force, dry_run)?;

            // Analyse if any of the dependent dependencies have changed.
            for effect in view {
                outdated |= match effect.has_changed() {
                    Ok(outdated) => outdated,
                    Err(err)     => { return Err(TargetError::HasChangedError{ effect_name: effect.name().into(), err }); }
                };
            }
        }

        // Done, everything is built, but we only return if outdated if any effect has been changed or if we `--force`ed.
        Ok(outdated)
    }

    /// Commits any changes to our own effects to the cache (or whatever we use to keep track of changes).
    /// 
    /// # Arguments
    /// - `dry_run`: If 'true', prints what would be done instead of actually executing the commands. Note that this is an imperfect simulation, since effect changes cannot be accurately detected without actually changing them.
    /// 
    /// # Errors
    /// This function errors if we failed to commit any of our own effects.
    fn commit(&self, dry_run: bool) -> Result<(), TargetError> {
        // Go through our own effects and update 'em
        for effect in self.effects() {
            if let Err(err) = effect.commit_change(dry_run) { return Err(TargetError::CommitError{ effect_name: effect.name().into(), err }); }
        }

        // Done
        Ok(())
    }



    // Child-provided
    /// Builds this Target as it likes.
    /// 
    /// You can assume that this function is only called if the dependencies have been build _and_ produced any changes in the effects that we depend upon.
    /// 
    /// After this operation, it will be safe to call `Target::commit()`.
    /// 
    /// # Arguments
    /// - `dry_run`: If 'true', prints what would be done instead of actually executing the commands. Note that this is an imperfect simulation, since effect changes cannot be accurately detected without actually changing them.
    /// 
    /// # Errors
    /// This function errors if we failed to build this target.
    fn build(&self, dry_run: bool) -> Result<(), TargetError>;



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
