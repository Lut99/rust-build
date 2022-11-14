//  VIEW.rs
//    by Lut99
// 
//  Created:
//    13 Nov 2022, 16:27:39
//  Last edited:
//    13 Nov 2022, 16:42:42
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements a so-called EffectView, which filters down the
//!   effects produced by another target to a subset of all of the
//!   effects it produces. This is useful when you want to depend on only a
//!   subset of effects produced by a target.
// 

use crate::spec::{Effect, Target};


/***** AUXILLARY *****/
/// Defines a ViewFilter, which is used to filter Target Effects when depending on them.
#[derive(Clone)]
pub enum ViewFilter {
    /// Lets no effects pass (filters them all out).
    None,
    /// Lets all effects pass (filters none of them out).
    All,

    /// Applies a whitelist of names for effects to pass.
    Allow{ names: Vec<String> },
    /// Applies a blacklist of names for effects to block.
    Deny{ names: Vec<String> },
}

impl ViewFilter {
    /// Checks if the given Effect would make it through this filter.
    /// 
    /// # Arguments
    /// - `effect`: The Effect to filter.
    /// 
    /// # Returns
    /// true if the effect still passes the filters, or false otherwise.
    pub fn filter(&self, effect: &dyn Effect) -> bool {
        use ViewFilter::*;
        match self {
            None => false,
            All  => true,

            Allow{ names } => {
                for n in names {
                    if n == effect.name() { return true; }
                }
                false
            },
            Deny{ names } => {
                for n in names {
                    if n == effect.name() { return false; }
                }
                true
            }
        }
    }
}



/// Defines a consuming iterator over an EffectView.
pub struct EffectViewIntoIter<'a> {
    /// The parent iterator of effects to iterator over.
    iter    : std::slice::Iter<'a, Box<dyn Effect>>,
    /// The list of filters to apply.
    filters : Vec<ViewFilter>,
}
impl<'a> Iterator for EffectViewIntoIter<'a> {
    type Item = &'a Box<dyn Effect>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Get the next item
            let next: &'a Box<dyn Effect> = match self.iter.next() {
                Some(next) => next,
                None       => { return None; },
            };

            // Apply the filters
            let mut allowed: bool = true;
            for f in &self.filters {
                if !f.filter(next.as_ref()) {
                    allowed = false;
                    break;
                }
            }
            if !allowed { continue; }

            // Return it if we made it through
            return Some(next);
        }
    }
}

/// Defines an iterator over an EffectView.
pub struct EffectViewIter<'a, 'b> {
    /// The parent iterator of effects to iterator over.
    iter    : std::slice::Iter<'a, Box<dyn Effect>>,
    /// The list of filters to apply.
    filters : &'b [ViewFilter],
}
impl<'a, 'b> Iterator for EffectViewIter<'a, 'b> {
    type Item = &'a Box<dyn Effect>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Get the next item
            let next: &'a Box<dyn Effect> = match self.iter.next() {
                Some(next) => next,
                None       => { return None; },
            };

            // Apply the filters
            let mut allowed: bool = true;
            for f in self.filters {
                if !f.filter(next.as_ref()) {
                    allowed = false;
                    break;
                }
            }
            if !allowed { continue; }

            // Return it if we made it through
            return Some(next);
        }
    }
}

// /// Defines a mutable iterator over an EffectView.
// pub struct EffectViewIterMut<'a, 'b> {
//     /// The parent iterator of effects to iterator over.
//     iter    : std::slice::IterMut<'a, Box<dyn Effect>>,
//     /// The list of filters to apply.
//     filters : &'b [ViewFilter],
// }
// impl<'a, 'b> Iterator for EffectViewIterMut<'a, 'b> {
//     type Item = &'a mut Box<dyn Effect>;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             // Get the next item
//             let next: &'a mut Box<dyn Effect> = match self.iter.next() {
//                 Some(next) => next,
//                 None       => { return None; },
//             };

//             // Apply the filters
//             let mut allowed: bool = true;
//             for f in self.filters {
//                 if !f.filter(next.as_ref()) {
//                     allowed = false;
//                     break;
//                 }
//             }
//             if !allowed { continue; }

//             // Return it if we made it through
//             return Some(next);
//         }
//     }
// }





/***** LIBRARY *****/
/// Defines an EffectView, which is a specific view on a Target's effects that another dependency has (so it doesn't have to dependent on all of its files).
#[derive(Clone)]
pub struct EffectView<'a> {
    /// The parent target that we view.
    pub(crate) target  : &'a dyn Target,
    /// The list of filters to apply.
    pub(crate) filters : Vec<ViewFilter>,
}

impl<'a> EffectView<'a> {
    /// Adds a new filter to the view that can be used to restrict which effects we see.
    /// 
    /// When thinking about filters, think about a stream of effects. Every filter is then some operation to filter out some effects and keep others. Thus, the order of filters matter (since they are applied as a pipeline).
    /// 
    /// # Arguments
    /// - `filter`: The ViewFilter to apply.
    /// 
    /// # Returns
    /// The same TargetView as went in for chaining purposes.
    #[inline]
    pub fn add_filter(self, filter: ViewFilter) -> Self {
        let mut this = self;
        this.filters.push(filter);
        this
    }



    /// Returns an iterator over the surviving effects after all filters have been applied.
    #[inline]
    pub fn iter<'b>(&'b self) -> EffectViewIter<'a, 'b> { self.into_iter() }
}

impl<'a> IntoIterator for EffectView<'a> {
    type Item     = &'a Box<dyn Effect>;
    type IntoIter = EffectViewIntoIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        EffectViewIntoIter {
            iter    : self.target.effects().iter(),
            filters : self.filters,
        }
    }
}
impl<'a, 'b> IntoIterator for &'b EffectView<'a> {
    type Item     = &'a Box<dyn Effect>;
    type IntoIter = EffectViewIter<'a, 'b>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        EffectViewIter {
            iter    : self.target.effects().iter(),
            filters : &self.filters,
        }
    }
}
