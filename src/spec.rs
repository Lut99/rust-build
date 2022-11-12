//  SPEC.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:01:47
//  Last edited:
//    12 Nov 2022, 13:51:05
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


/***** LIBRARY *****/
/// Defines a Depedency, which is some kind of object that has to perform some action before a subsequent Target can be run.
pub trait Dependency {
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



/// Defines a Target, which is something that compiles, installs or runs something else.
pub trait Target {
    
}



/// Defines an Effect, which is something that a Target produces. Typically (though not always), an Effect is also a Dependency such that future target may use it themselves.
pub trait Effect {
    
}
