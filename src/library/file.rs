//  FILE.rs
//    by Lut99
// 
//  Created:
//    12 Nov 2022, 13:44:39
//  Last edited:
//    13 Nov 2022, 16:31:48
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines file-related effects, targets and dependencies.
// 

use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;
use std::rc::Rc;

use crate::{trace, warn};
use crate::spec::{Dependency, Effect, Named};
use crate::cache::{Cache, CacheEntry, LastEditedTime};


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
    /// The name of this file.
    name  : String,
    /// The Cache that we use to discover if the file has changed since last checks.
    cache : Rc<Cache>,

    /// The path of the file this Effect concerns itself about.
    pub path : PathBuf,
}

impl File {
    /// Constructor for the File dependency.
    /// 
    /// # Arguments
    /// - `name`: The name of this File.
    /// - `cache`: The Cache to use to keep track of this file's changed status.
    /// - `path`: The path of the file that this dependency tracks.
    /// 
    /// # Returns
    /// A new File instance.
    #[inline]
    pub fn new(name: impl Into<String>, cache: Rc<Cache>, path: impl Into<PathBuf>) -> Self {
        Self {
            name : name.into(),
            cache,

            path : path.into(),
        }
    }
}

impl Named for File {
    #[inline]
    fn name(&self) -> &str { &self.name }
}

impl Dependency for File {
    fn has_changed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        // Check if the file exists
        if !self.path.exists() { return Err(Box::new(Error::FileNotFound{ path: self.path.clone() })); }

        // Check if the cache file exists
        let entry: CacheEntry = match self.cache.get_file(&self.path) {
            Ok(Some(entry)) => entry,
            Ok(None)        => {
                trace!("Marking '{}' as changed (no cache entry found)", self.path.display());
                return Ok(true);
            },
            Err(err) => { return Err(Box::new(err)); },
        };

        // If it does, fetch the file's most recent change date
        let last_edited: LastEditedTime = match LastEditedTime::from_path(&self.path) {
            Ok(last_edited) => last_edited,
            Err(err)        => { return Err(Box::new(err)); },
        };

        // Check if it's needed to recompile
        if entry.last_edited > last_edited {
            warn!("Last edited time in the cache is later than on disk; that seems weird (assuming recompilation is needed)");
            trace!("Marking '{}' as changed (invalid cached time)", self.path.display());
            Ok(true)
        } else {
            #[cfg(feature = "log")]
            if entry.last_edited != last_edited {
                trace!("Marking '{}' as unchanged (same last edited time as in cache)", self.path.display());
            } else {
                trace!("Marking '{}' as changed (last edited time later than in cache)", self.path.display());
            }
            Ok(entry.last_edited != last_edited)
        }
    }
}

impl Effect for File {
    fn commit_change(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the file exists
        if !self.path.exists() { return Err(Box::new(Error::FileNotFound{ path: self.path.clone() })); }

        // Fetch the current last edited file
        let last_edited: LastEditedTime = match LastEditedTime::from_path(&self.path) {
            Ok(last_edited) => last_edited,
            Err(err)        => { return Err(Box::new(err)); },
        };

        // Write the last edited date to the cache
        match self.cache.update_file(&self.path, CacheEntry {
            last_edited,
        }) {
            Ok(_)    => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }
}
