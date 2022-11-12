//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:00:31
//  Last edited:
//    12 Nov 2022, 14:22:29
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines errors for the `rust-build` crate.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};
use std::path::PathBuf;


/***** LIBRARY *****/
/// The toplevel error of the crate.
#[derive(Debug)]
pub enum BuildError {
    Temp,
}

impl Display for BuildError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use BuildError::*;
        match self {
            Temp => write!(f, "TEMP"),
        }
    }
}

impl Error for BuildError {}



/// Defines errors that relate to the Cache.
#[derive(Debug)]
pub enum CacheError {
    /// The given path did not exist.
    CacheDirNotFound{ path: PathBuf },
    /// The given path existed but was not a directory.
    CacheDirNotADir{ path: PathBuf },
    /// Failed to create a new directory.
    CacheDirCreateError{ path: PathBuf, err: std::io::Error },

    /// The given path existed but was not a file.
    CacheEntryNotAFile{ path: PathBuf, },
    /// Failed to open the given cache entry.
    CacheEntryOpenError{ path: PathBuf, err: std::io::Error },
    /// Failed to parse the given cache entry.
    CacheEntryParseError{ path: PathBuf, err: serde_json::Error },
}

impl Display for CacheError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use CacheError::*;
        match self {
            CacheDirNotFound{ path }         => write!(f, "Given make cache directory '{}' does not exist", path.display()),
            CacheDirNotADir{ path }          => write!(f, "Given make cache directory '{}' exists but is not a directory", path.display()),
            CacheDirCreateError{ path, err } => write!(f, "Failed to create make cache directory '{}': {}", path.display(), err),

            CacheEntryNotAFile{ path }        => write!(f, "Given make cache entry '{}' exists but is not a file", path.display()),
            CacheEntryOpenError{ path, err }  => write!(f, "Failed to open cache entry file '{}': {}", path.display(), err),
            CacheEntryParseError{ path, err } => write!(f, "Failed to read and parse cache entry file '{}' as JSON: {}", path.display(), err),
        }
    }
}

impl Error for CacheError {}
