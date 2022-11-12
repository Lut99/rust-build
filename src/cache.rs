//  CACHE.rs
//    by Lut99
// 
//  Created:
//    12 Nov 2022, 13:47:41
//  Last edited:
//    12 Nov 2022, 14:19:52
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the installer's own build cache, which can keep track of
//!   various things.
// 

use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::debug;
pub use crate::errors::CacheError as Error;


/***** LIBRARY *****/
/// The CacheEntry struct provides cached information about a build file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheEntry {
    
}





/// The Cache struct is used to interact with the build cache, which stores information about whether things have been updated since last calls.
#[derive(Clone, Debug)]
pub struct Cache {
    /// The path where this cache lives.
    path : PathBuf,
}

impl Cache {
    /// Constructor for the Cache.
    /// 
    /// # Arguments
    /// - `path`: The path to the build cache directory that we will use / have used last time. Obviously, it should make sense to try and keep this in the same location.
    /// - `create_path`: Whether to attempt to create the directory if it does not exist (true) or just error instead (false).
    /// 
    /// # Returns
    /// A new Cache instance.
    /// 
    /// # Errors
    /// This function errors if any sanity checks about the path failed (whether it exists and is a directory and such).
    pub fn new(path: impl Into<PathBuf>, create_path: bool) -> Result<Self, Error> {
        let path: PathBuf = path.into();

        // Do some path sanity checks
        if !path.exists() {
            // Create it or sniff it
            if create_path {
                if let Err(err) = fs::create_dir_all(&path) { return Err(Error::CacheDirCreateError { path, err }); }
            } else {
                return Err(Error::CacheDirNotFound { path });
            }
        }
        if !path.is_dir() {
            return Err(Error::CacheDirNotADir { path });
        }

        // It checks out
        debug!("Cache location at: '{}'", path.display());
        Ok(Self {
            path : path.into(),
        })
    }



    /// A bit of an odd function that hashes a given source identifier to a cache identifier.
    /// 
    /// # Arguments
    /// - `source`: The source identifier (i.e., path, Docker image name, ...) to convert into a proper cache ID.
    /// 
    /// # Returns
    /// The hash of the path, as a raw u64 number.
    pub fn hash(source: impl Hash) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }



    /// Returns the cache entry for the given file if there is any.
    /// 
    /// # Arguments
    /// - `file`: The file to cache. Note that its path acts as a unique identifier.
    /// 
    /// # Returns
    /// The CacheEntry if we were able to find one. Otherwise, returns `None`.
    /// 
    /// # Errors
    /// This function errors if the make cache was ill-formed or if we encounter disk IO errors.
    pub fn get_file(&self, file: impl AsRef<Path>) -> Result<Option<CacheEntry>, Error> {
        let file: &Path = file.as_ref();

        // Hash the filename to use as identifier
        let hash  : u64    = Self::hash(file);
        let shash : String = format!("{}", hash);
        debug!("File '{}' ID: {}", file.display(), shash);

        // Attempt to find the file with that information
        let file_path: PathBuf = self.path.join(shash);
        if !file_path.exists() { return Ok(None); }
        if !file_path.is_file() { return Err(Error::CacheEntryNotAFile{ path: file_path }); }

        // Attempt to read it using serde
        match File::open(&file_path) {
            Ok(handle) => match serde_json::from_reader(handle) {
                Ok(entry) => Ok(Some(entry)),
                Err(err)  => Err(Error::CacheEntryParseError{ path: file_path, err }),
            },
            Err(err) => Err(Error::CacheEntryOpenError{ path: file_path, err }),
        }
    }
}
