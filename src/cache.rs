//  CACHE.rs
//    by Lut99
// 
//  Created:
//    12 Nov 2022, 13:47:41
//  Last edited:
//    13 Nov 2022, 14:47:21
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the installer's own build cache, which can keep track of
//!   various things.
// 

use std::collections::hash_map::DefaultHasher;
use std::fmt::{Formatter, Result as FResult};
use std::fs::{self, File, Metadata};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use filetime::FileTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};
use serde::ser::SerializeSeq;

use crate::debug;
pub use crate::errors::{CacheError as Error, LastEditedTimeError};


/***** LIBRARY *****/
/// Defines a custom wrapper around a FileTime to implement serialize & deserialize for it.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LastEditedTime(FileTime);

impl LastEditedTime {
    /// Constructor for the LastEditedTime that retrieves it from the given file.
    /// 
    /// # Arguments
    /// - `path`: The path of the file or directory to retrieve the last edited time for.
    /// 
    /// # Returns
    /// A new LastEditedTime instance that represents the most recent point in time the given file was edited.
    /// 
    /// # Errors
    /// This function may error if the given file doesn't exist or couldn't be read.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, LastEditedTimeError> {
        let path: &Path = path.as_ref();

        // Read the metadata
        let metadata: Metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(err)     => {
                return Err(if err.kind() == std::io::ErrorKind::NotFound {
                    LastEditedTimeError::PathNotFound { path: path.into() }
                } else {
                    LastEditedTimeError::PathMetadataReadError { path: path.into(), err }
                });
            },
        };

        // Return the LastEditedTime
        Ok(Self(FileTime::from_last_modification_time(&metadata)))
    }
}

impl Serialize for LastEditedTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // We serialize it as tuple of numbers
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.0.unix_seconds())?;
        seq.serialize_element(&self.0.nanoseconds())?;
        seq.end()
    }
}
impl<'de> Deserialize<'de> for LastEditedTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        // Define a visitor for the LastEditedTime
        struct LastEditedTimeVisitor;
        impl<'de> Visitor<'de> for LastEditedTimeVisitor {
            type Value = LastEditedTime;

            fn expecting(&self, f: &mut Formatter) -> FResult {
                write!(f, "a last edited time")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut seq = seq;

                // Simply get the two elements, in order
                let secs : i64 = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nano : u32 = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(LastEditedTime(FileTime::from_unix_time(secs, nano)))
            }
        }

        // Visit it
        deserializer.deserialize_seq(LastEditedTimeVisitor)
    }
}

impl AsRef<LastEditedTime> for LastEditedTime {
    #[inline]
    fn as_ref(&self) -> &LastEditedTime {
        self
    }
}
impl AsMut<LastEditedTime> for LastEditedTime {
    #[inline]
    fn as_mut(&mut self) -> &mut LastEditedTime {
        self
    }
}
impl From<&LastEditedTime> for LastEditedTime {
    #[inline]
    fn from(value: &LastEditedTime) -> Self {
        value.clone()
    }
}
impl From<&mut LastEditedTime> for LastEditedTime {
    #[inline]
    fn from(value: &mut LastEditedTime) -> Self {
        value.clone()
    }
}

impl From<FileTime> for LastEditedTime {
    #[inline]
    fn from(value: FileTime) -> Self {
        Self(value)
    }
}
impl From<&FileTime> for LastEditedTime {
    #[inline]
    fn from(value: &FileTime) -> Self {
        Self::from(value.clone())
    }
}
impl From<&mut FileTime> for LastEditedTime {
    #[inline]
    fn from(value: &mut FileTime) -> Self {
        Self::from(value.clone())
    }
}

impl From<LastEditedTime> for FileTime {
    #[inline]
    fn from(value: LastEditedTime) -> Self {
        value.0
    }
}
impl From<&LastEditedTime> for FileTime {
    #[inline]
    fn from(value: &LastEditedTime) -> Self {
        Self::from(value.clone())
    }
}
impl From<&mut LastEditedTime> for FileTime {
    #[inline]
    fn from(value: &mut LastEditedTime) -> Self {
        Self::from(value.clone())
    }
}

impl AsRef<FileTime> for LastEditedTime {
    fn as_ref(&self) -> &FileTime {
        &self.0
    }
}
impl AsMut<FileTime> for LastEditedTime {
    fn as_mut(&mut self) -> &mut FileTime {
        &mut self.0
    }
}
impl Deref for LastEditedTime {
    type Target = FileTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for LastEditedTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}





/// The CacheEntry struct provides cached information about a build file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheEntry {
    /// The last time the file was edited.
    pub last_edited : LastEditedTime,
}

impl AsRef<CacheEntry> for CacheEntry {
    #[inline]
    fn as_ref(&self) -> &CacheEntry {
        self
    }
}
impl AsMut<CacheEntry> for CacheEntry {
    #[inline]
    fn as_mut(&mut self) -> &mut CacheEntry {
        self
    }
}
impl From<&CacheEntry> for CacheEntry {
    #[inline]
    fn from(value: &CacheEntry) -> Self {
        value.clone()
    }
}
impl From<&mut CacheEntry> for CacheEntry {
    #[inline]
    fn from(value: &mut CacheEntry) -> Self {
        value.clone()
    }
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
        debug!("get_file(): File '{}' ID: {}", file.display(), shash);

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

    /// Updates the cache entry for a given file if there is any.
    /// 
    /// # Arguments
    /// - `file`: The file to update the cache for. Note that its path acts as a unique identifier.
    /// - `info`: The CacheEntry with the info to update the file to.
    /// 
    /// # Errors
    /// This function errors if we failed to update the cache entry. This is typically due to IO errors.
    pub fn update_file(&self, file: impl AsRef<Path>, info: impl AsRef<CacheEntry>) -> Result<(), Error> {
        let file : &Path       = file.as_ref();
        let info : &CacheEntry = info.as_ref();

        // Hash the filename to use as identifier
        let hash  : u64    = Self::hash(file);
        let shash : String = format!("{}", hash);
        debug!("update_file(): File '{}' ID: {}", file.display(), shash);

        // Attempt to write the cache entry to that file
        let file_path: PathBuf = self.path.join(shash);
        match File::create(&file_path) {
            Ok(handle) => match serde_json::to_writer(handle, info) {
                Ok(_)    => Ok(()),
                Err(err) => Err(Error::CacheEntryWriteError{ path: file_path, err }),
            },
            Err(err) => Err(Error::CacheEntryCreateError{ path: file_path, err }),
        }
    }
}
