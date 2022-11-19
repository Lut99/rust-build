//  SPEC.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:01:47
//  Last edited:
//    19 Nov 2022, 11:54:02
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
use std::rc::Rc;

use crate::errors::TargetError;
use crate::view::{EffectView, ViewFilter};
use crate::cache::Cache;


/***** LIBRARY *****/
/// Defines target operating systems to build for.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OperatingSystem {
    /// Windows operating system
    Windows,
    /// macOS operating system
    MacOs,
    /// Linux operating system
    Linux,

    /// A custom OS ID usable by custom targets.
    Custom(&'static str),
}
impl OperatingSystem {
    /// Returns the default OperatingSystem that we're running on.
    /// 
    /// Note that it's actually deduced based on compile-time constants, making this function constant too - but also possible inaccurate if you ever need to depend on what the OS reports.
    /// 
    /// # Returns
    /// The operating system of the current host.
    #[inline]
    #[cfg(target_os = "windows")]
    pub const fn host() -> Self { Self::Windows }
    #[cfg(target_os = "macos")]
    pub const fn host() -> Self { Self::MacOs }
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    pub const fn host() -> Self { Self::Linux }
    #[cfg(not(any(target_os = "windows", target_os = "macos", all(target_family = "unix", not(target_os = "macos")))))]
    pub const fn host() -> Self { Self::custom("unknown") }
}

/// Defines target architectures to build for.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Architecture {
    /// Classic x86, 32-bit
    #[allow(non_camel_case_types)]
    x86_32,
    /// Classic x86, 64-bit
    #[allow(non_camel_case_types)]
    x86_64,

    /// ARM 32-bit
    Aarch32,
    /// Arm 64-bit
    Aarch64,

    /// Power PC 32-bit
    PowerPc32,
    /// Power PC 64-bit
    PowerPc64,

    /// MIPS
    Mips,

    /// A custom architecture ID usable by custom targets.
    Custom(&'static str),
}
impl Architecture {
    /// Returns the default Architecture that we're running on.
    /// 
    /// Note that it's actually deduced based on compile-time constants, making this function constant too - but also possible inaccurate if you ever need to depend on what the OS reports.
    /// 
    /// # Returns
    /// The architecture of the current host.
    #[inline]
    #[cfg(target_arch = "x86")]
    pub const fn host() -> Self { Self::x86_32 }
    #[cfg(target_arch = "x86_64")]
    pub const fn host() -> Self { Self::x86_64 }
    #[cfg(target_arch = "arm")]
    pub const fn host() -> Self { Self::Aarch32 }
    #[cfg(target_arch = "aarch64")]
    pub const fn host() -> Self { Self::Aarch64 }
    #[cfg(target_arch = "powerpc")]
    pub const fn host() -> Self { Self::PowerPc32 }
    #[cfg(target_arch = "powerpc64")]
    pub const fn host() -> Self { Self::PowerPc64 }
    #[cfg(target_arch = "mips")]
    pub const fn host() -> Self { Self::Mips }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "mips")))]
    pub const fn host() -> Self { Self::Custom("unknown") }
}





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
    /// - `target`: The BuildTarget to build for.
    /// - `os`: The target OS that we intend to build.
    /// - `arch`: The target architecture that we intend to build.
    /// - `force`: If 'true', always builds all targets instead of only when there is no (detected) change.
    /// - `dry_run`: If 'true', prints what would be done instead of actually executing the commands. Note that this is an imperfect simulation, since effect changes cannot be accurately detected without actually changing them.
    /// 
    /// # Errors
    /// This function errors if any of the three other functions would error.
    fn make(&self, os: OperatingSystem, arch: Architecture, force: bool, dry_run: bool) -> Result<(), TargetError> {
        // Call the dependencies first, to find out if anything has to happen.
        let outdated: bool = self.build_deps(os, arch, force, dry_run)?;

        // Next, if it does, run the build & commit
        if outdated {
            self.build(os, arch, dry_run)?;
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
    /// - `os`: The target OS that we intend to build.
    /// - `arch`: The target architecture that we intend to build.
    /// - `force`: If 'true', always builds all dependencies instead of only when there is no (detected) change to their dependencies.
    /// - `dry_run`: If 'true', prints what would be done instead of actually executing the commands. Note that this is an imperfect simulation, since effect changes cannot be accurately detected without actually changing them.
    /// 
    /// # Returns
    /// Whether any of the resulting cache files is outdated or not, and thus whether this Target should be rebuild or not. If `force` is true, then this also always returns true.
    /// 
    /// # Errors
    /// This function errors if we failed to build any of the targets this target depends on.
    fn build_deps(&self, os: OperatingSystem, arch: Architecture, force: bool, dry_run: bool) -> Result<bool, TargetError> {
        // Iterate over all of the views
        let mut outdated: bool = force;
        for view in self.deps() {
            // Build the target behind this view first.
            view.target.make(os, arch, force, dry_run)?;

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
    /// - `os`: The target OS that we intend to build.
    /// - `arch`: The target architecture that we intend to build.
    /// - `dry_run`: If 'true', prints what would be done instead of actually executing the commands. Note that this is an imperfect simulation, since effect changes cannot be accurately detected without actually changing them.
    /// 
    /// # Errors
    /// This function errors if we failed to build this target.
    fn build(&self, os: OperatingSystem, arch: Architecture, dry_run: bool) -> Result<(), TargetError>;



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



/// Defines a TargetBuilder, which is a common interface to all builders for targets.
pub trait TargetBuilder<'a> {
    /// The Target that we will build.
    type Target: Target;


    /// Constructor for the builder that initializes it to a default state for all targets.
    /// 
    /// # Arguments
    /// - `name`: The name of the target to build.
    /// 
    /// # Returns
    /// A new instance of Self.
    fn new(name: impl Into<String>) -> Self;



    /// Adds a single dependency to this TargetBuilder.
    /// 
    /// Specifically, adds a view for a dependency such that we may depend on only a part of its effects.
    /// 
    /// Note that any sanity checks won't be performed until `TargetBuilder::build()`.
    /// 
    /// # Arguments
    /// - `dep`: The EffectView that represents the parts of the dependency we care about.
    /// 
    /// # Returns
    /// The same `self` as given for chaining purposes.
    fn dep(self, dep: EffectView<'a>) -> Self;
    /// Adds a whole list of dependencies to this TargetBuilder.
    /// 
    /// Specifically, adds views for those dependency such that we may depend on only a part of its effects.
    /// 
    /// Note that any sanity checks won't be performed until `TargetBuilder::build()`.
    /// 
    /// # Arguments
    /// - `deps`: An iterator with EffectViews that represent the parts of the dependencies we care about.
    /// 
    /// # Returns
    /// The same `self` as given for chaining purposes.
    fn deps(self, deps: impl IntoIterator<Item = EffectView<'a>, IntoIter = impl Iterator<Item = EffectView<'a>>>) -> Self;

    /// Adds a single effect to this TargetBuilder.
    /// 
    /// Note that any sanity checks won't be performed until `TargetBuilder::build()`.
    /// 
    /// # Arguments
    /// - `effect`: The Effect that is something that this Target produces (and dependencies might care about).
    /// 
    /// # Returns
    /// The same `self` as given for chaining purposes.
    fn effect(self, effect: impl 'static + Effect) -> Self;
    /// Adds a whoe list effects to this TargetBuilder.
    /// 
    /// Note that any sanity checks won't be performed until `TargetBuilder::build()`.
    /// 
    /// # Arguments
    /// - `effect`: An iterator with Effects that are things that this Target produces (and dependencies might care about).
    /// 
    /// # Returns
    /// The same `self` as given for chaining purposes.
    fn effects(self, effects: impl IntoIterator<Item = impl 'static + Effect, IntoIter = impl Iterator<Item = impl 'static + Effect>>) -> Self;



    /// Builds the Target that this TargetBuilder can build.
    /// 
    /// # Arguments
    /// - `cache`: The Cache that may be used by targets to keep track of what has changed and how.
    /// 
    /// # Returns
    /// A new instance of the target Target.
    /// 
    /// # Panics
    /// Note that this function may panic due to any of the other factory methods producing invalid targets.
    fn build(self, cache: Rc<Cache>) -> Result<Self::Target, Box<dyn Error>>;
}
