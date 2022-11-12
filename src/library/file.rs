//  FILE.rs
//    by Lut99
// 
//  Created:
//    12 Nov 2022, 13:44:39
//  Last edited:
//    12 Nov 2022, 14:27:13
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines file-related effects, targets and dependencies.
// 

use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;
use std::rc::Rc;

use crate::spec::{Dependency, Effect, Target};
use crate::cache::{Cache, CacheEntry};


/***** ERRORS *****/
/// Defines errors that relate to the File.
#[derive(Debug)]
pub enum Error {
    /// The file was not found
    FileNotFound{ path: PathBuf },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            FileNotFound{ path } => write!(f, "Dependency file '{}' not found (did a previous target fail?)", path.display()),
        }
    }
}

impl std::error::Error for Error {}





/***** LIBRARY *****/
/// A File is both a Dependency and an Effect. It can be though of as a particular file that may be updated or changed by some target.
#[derive(Debug, Clone)]
pub struct File {
    /// The Cache that we use to discover if the file has changed since last checks.
    cache : Rc<Cache>,

    /// The path of the file this Effect concerns itself about.
    pub path : PathBuf,
}

impl File {
    /// Constructor for the File dependency.
    /// 
    /// # Arguments
    /// - `cache`: The Cache to use to keep track of this file's changed status.
    /// - `path`: The path of the file that this dependency tracks.
    /// 
    /// # Returns
    /// A new File instance.
    #[inline]
    pub fn new(cache: Rc<Cache>, path: impl Into<PathBuf>) -> Self {
        Self {
            cache,

            path : path.into(),
        }
    }
}

impl Dependency for File {
    fn has_changed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        // Check if the file exists
        if !self.path.exists() { return Err(Box::new(Error::FileNotFound{ path: self.path.clone() })); }

        // Check if the cache file exists
        let entry: CacheEntry = match self.cache.get_file(&self.path) {
            Ok(Some(entry)) => entry,
            Ok(None)        => { return Ok(true); },
            Err(err)        => { return Err(Box::new(err)); },
        };

        // If it does, fetch the file's most recent change date
        
    }
}

impl Effect for File {
    
}
