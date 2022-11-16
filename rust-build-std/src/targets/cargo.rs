//  CARGO.rs
//    by Lut99
// 
//  Created:
//    13 Nov 2022, 14:34:33
//  Last edited:
//    16 Nov 2022, 18:19:20
//  Auto updated?
//    Yes
// 
//  Description:
//!   Provides a target for compiling Rust with some default options.
//! 
//!   Note that this Target uses the `File` dependency/effect, also
//!   provided in the standard library.
// 

use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use toml::Value;

use rust_build::errors::TargetError;
use rust_build::spec::{Effect, Named, Target};
use rust_build::view::EffectView;


/***** ERRORS *****/
/// Defines errors that are CargoTarget-specific.
#[derive(Debug)]
pub enum Error {
    /// An expected Cargo.toml file did not exist.
    MissingCargoToml{ path: PathBuf },
    /// Failed to open a Cargo.toml file.
    CargoTomlOpenError{ path: PathBuf, err: std::io::Error },
    /// Failed to read a Cargo.toml file.
    CargoTomlReadError{ path: PathBuf, err: std::io::Error },
    /// Failed to parse a Cargo.toml file.
    CargoTomlParseError{ path: PathBuf, err: toml::de::Error },

    /// The given Cargo.toml file did not have a table in its toplevel.
    CargoTomlNotATable{ path: PathBuf },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            MissingCargoToml{ path }         => write!(f, "Missing Cargo.toml file '{}'", path.display()),
            CargoTomlOpenError{ path, err }  => write!(f, "Failed to open Cargo.toml file '{}': {}", path.display(), err),
            CargoTomlReadError{ path, err }  => write!(f, "Failed to read Cargo.toml file '{}': {}", path.display(), err),
            CargoTomlParseError{ path, err } => write!(f, "Failed to parse Cargo.toml file '{}': {}", path.display(), err),

            CargoTomlNotATable{ path } => write!(f, "'{}' does not have a toplevel table.", path.display()),
        }
    }
}

impl std::error::Error for Error {}





/***** LIBRARY *****/
/// Defines the Cargo target, which uses the Cargo build system to compile Rust code.
/// 
/// This can typically be used as a starting point in your dependency tree.
pub struct CargoTarget<'a> {
    /// The name of this target.
    name    : String,
    /// The dependencies of this target.
    deps    : Vec<EffectView<'a>>,
    /// The effects (that we care about) of this target.
    effects : Vec<Box<dyn Effect>>,

    /// The path of the directory where the target package (or workspace) lives.
    path     : PathBuf,
    /// The packages that we build in this run.
    packages : Vec<String>,
}

impl<'a> CargoTarget<'a> {
    /// Constructor for the CargoTarget.
    /// 
    /// Note that the source files and effects of this target will actually be deduced based on the `Cargo.toml` file we assume to be present in the given folder.
    /// 
    /// # Arguments
    /// - `name`: The name/identifier of this target.
    /// - `path`: The path to the directory with the package (or workspace).
    /// 
    /// # Returns
    /// A new CargoTarget instance.
    /// 
    /// # Errors
    /// This function errors if we failed to deduce packages or effects from the given folder.
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Result<Self, Error> {
        let name: String  = name.into();
        let path: PathBuf = path.into();

        // Deduce the things
        let effects  : Vec<Box<dyn Effect>> = Self::deduce_effects(&path)?;
        let packages : Vec<String>          = Self::deduce_packages(&path)?;

        // Return the new Target
        Ok(Self {
            name,
            deps : vec![],
            effects,

            path,
            packages,
        })
    }



    /// Deduces the list of effects from either the given package or workspace directory by inspecting the Cargo.toml.
    /// 
    /// If the path points to a package, the resulting binary file (either the lib or name) read from the Cargo.toml or deduced is returned.
    /// 
    /// Otherwise, it recursively collects resulting binaries from each package in the workspace.
    /// 
    /// # Arguments
    /// - `path`: The path to the directory with the package (or workspace).
    /// 
    /// # Returns
    /// A vector of effects, each of which is the (relevant) output file(s) of a package.
    /// 
    /// # Errors
    /// This function errors if we failed to find, read or parse the `Cargo.toml` file.
    pub fn deduce_effects(path: impl AsRef<Path>) -> Result<Vec<Box<dyn Effect>>, Error> {
        Ok(vec![])
    }

    /// Deduces the list of packages from either the given package or workspace directory by inspecting the Cargo.toml.
    /// 
    /// If the path points to a package, the name of the package is returned as only package.
    /// 
    /// Otherwise, the list of members is returned.
    /// 
    /// # Arguments
    /// - `path`: The path to the directory with the package (or workspace).
    /// 
    /// # Returns
    /// A vector of strings, each of which is the name of a package in this directory.
    /// 
    /// # Errors
    /// This function errors if we failed to find, read or parse the `Cargo.toml` file.
    pub fn deduce_packages(path: impl AsRef<Path>) -> Result<Vec<String>, Error> {
        let path: &Path = path.as_ref();

        // Attempt to open the Cargo.toml file and read its contents
        let cargo_path: PathBuf = path.join("Cargo.toml");
        let cargo_toml: Vec<u8> = match File::open(&cargo_path) {
            Ok(mut handle) => {
                let mut res: Vec<u8> = vec![];
                match handle.read_to_end(&mut res) {
                    Ok(_)    => res,
                    Err(err) => { return Err(Error::CargoTomlReadError{ path: cargo_path, err }); },
                }
            },
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound { return Err(Error::MissingCargoToml { path: path.into() }); }
                return Err(Error::CargoTomlOpenError{ path: cargo_path, err });
            }
        };

        // Parse it with serde (and toml)
        let cargo_toml: Value = match toml::from_slice(&cargo_toml) {
            Ok(cargo_toml) => cargo_toml,
            Err(err)       => { return Err(Error::CargoTomlParseError{ path: cargo_path, err }); },
        };

        // The file must be a toplevel table
        if let Value::Table(table) = cargo_toml {
            // If we find a workspace, assume that; otherwise, we get the parts of the package we are interested in
            

        } else {
            return Err(Error::CargoTomlNotATable{ path: cargo_path });
        }

        // Done
        Ok(vec![])
    }



    /// Returns the path to the directory where this target builds.
    #[inline]
    pub fn path(&self) -> &Path { &self.path }

    /// Returns the list of packages we're building.
    #[inline]
    pub fn packages(&self) -> &[String] { &self.packages }
}

impl<'a> Named for CargoTarget<'a> {
    #[inline]
    fn name(&self) -> &str { &self.name }
}
impl<'a> Target for CargoTarget<'a> {
    fn build(&self, dry_run: bool) -> Result<(), TargetError> {
        /* TODO */
        Ok(())
    }



    #[inline]
    fn deps(&self) -> &[EffectView] { &self.deps }

    #[inline]
    fn effects(&self) -> &[Box<dyn Effect>] { &self.effects }
}
