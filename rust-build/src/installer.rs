//  INSTALLER.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:13:20
//  Last edited:
//    16 Nov 2022, 18:13:11
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the main Installer class, which is used to build each of the
//!   individual installer components.
// 

use std::collections::HashMap;
use std::rc::Rc;

use crate::spec::Target;
use crate::style::InstallerStyle;


/***** LIBRARY *****/
/// Defines a builder for the installer.
pub struct Builder {
    /// The list of targets that we will build the installer with.
    targets : Vec<Box<dyn Target>>,
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Self {
            targets : vec![],
        }
    }
}

impl Builder {
    /// Adds a new target to the builder.
    /// 
    /// # Arguments
    /// - `target`: The Target to add.
    /// 
    /// # Returns
    /// The same `Builder` as self, for chaining purposes.
    /// 
    /// # Panics
    /// This function may cause panics in the `Builder::build()` function if the target's name conflicts with that of another target.
    #[inline]
    pub fn add_target(mut self, target: impl 'static + Target) -> Self {
        self.targets.push(Box::new(target));
        self
    }
}



/// Defines the Installer, which collects and has overview over all the targets and such.
pub struct Installer {
    /// Determines the style of the installer (i.e., the colour scheme and such).
    style : InstallerStyle,

    /// Keeps track of all of the targets registered in the Installer.
    targets : HashMap<String, Rc<dyn Target>>,
}

impl Installer {
    /// Returns a builder for the Installer that can be used to define it it.
    /// 
    /// # Returns
    /// A new Builder instance.
    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
    }



    // /// Registers a new build target with the installer.
    // /// 
    // /// # Arguments
    // /// - `target`: The Target to register.
    // /// 
    // /// # Returns
    // /// Nothing, but does register it internally.
    // /// 
    // /// # Panics
    // /// This function may panic if the given Target had a conflicting name with other, already established targets.
    // #[inline]
    // pub fn register(&mut self, target: impl Target) {
    //     // Sanity check the name's unique
    //     if let Some(old) = self.targets.insert(target.name().clone(), Rc::new(target)) {
    //         panic!("A Target with name '{}' is already registered", old.name());
    //     }
    // }
}
