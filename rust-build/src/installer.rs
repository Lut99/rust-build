//  INSTALLER.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:13:20
//  Last edited:
//    13 Nov 2022, 14:46:58
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
/// Defines the Installer, which collects and has overview over all the targets and such.
pub struct Installer {
    /// Determines the style of the installer (i.e., the colour scheme and such).
    style : InstallerStyle,

    /// Keeps track of all of the targets registered in the Installer.
    targets : HashMap<String, Rc<dyn Target>>,
}

impl Installer {
    /// Constructor for the Installer.
    /// 
    /// # Arguments
    /// - `style`: The InstallerStyle that determines which colours are used to print what.
    /// 
    /// # Returns
    /// A new Installer instance, ready for use in your own project.
    #[inline]
    pub fn new(style: InstallerStyle) -> Self {
        Self {
            style,

            targets : HashMap::new(),
        }
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

impl Default for Installer {
    #[inline]
    fn default() -> Self {
        Self::new(InstallerStyle::default())
    }
}
