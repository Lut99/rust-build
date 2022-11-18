//  CARGO.rs
//    by Lut99
// 
//  Created:
//    13 Nov 2022, 14:34:33
//  Last edited:
//    18 Nov 2022, 18:53:33
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
use toml::map::Map;

use crate::{debug, trace};
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

    /// The given Cargo.toml did not have a '[[bin]]' _or_ a '[package]' field.
    CargoTomlEffectsDeduceError{ path: PathBuf },
    /// The '[[bin]]'s are not an Array.
    CargoTomlBinsTypeError{ path: PathBuf, data_type: &'static str },
    /// One of the '[[bin]]'s is not a table.
    CargoTomlBinTypeError{ path: PathBuf, data_type: &'static str },
    /// The given Cargo.toml has a 'package' table, but not a nested 'name' field.
    CargoTomlMissingName{ table: &'static str, path: PathBuf },
    /// The 'name' field in the Cargo.toml was of an incorrect type.
    CargoTomlNameTypeError{ what: &'static str, path: PathBuf, data_type: &'static str },

    /// The given Cargo.toml had a 'workspace' table, but not a nested 'members' table.
    CargoTomlMissingMembers{ path: PathBuf },
    /// The 'members' field in the Cargo.toml was of an incorrect type.
    CargoTomlMembersTypeError{ path: PathBuf, data_type: &'static str },
    /// The 'members' list in the Cargo.toml had a non-String element
    CargoTomlMemberTypeError{ path: PathBuf, data_type: &'static str },
    /// The given Cargo.toml did not have a '[workspace]' _or_ a '[package]' field.
    CargoTomlPackagesDeduceError{ path: PathBuf },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            MissingCargoToml{ path }         => write!(f, "Missing Cargo.toml file '{}'", path.display()),
            CargoTomlOpenError{ path, err }  => write!(f, "Failed to open Cargo.toml file '{}': {}", path.display(), err),
            CargoTomlReadError{ path, err }  => write!(f, "Failed to read Cargo.toml file '{}': {}", path.display(), err),
            CargoTomlParseError{ path, err } => write!(f, "Failed to parse Cargo.toml file '{}': {}", path.display(), err),
            CargoTomlNotATable{ path }       => write!(f, "{}: No toplevel table found", path.display()),

            CargoTomlEffectsDeduceError{ path }             => write!(f, "{}: No '[[bin]]' or '[package]' toplevel table found", path.display()),
            CargoTomlBinsTypeError{ path, data_type }       => write!(f, "{}: Expected an Array as '[[bin]]'s, but got {}", path.display(), data_type),
            CargoTomlBinTypeError{ path, data_type }        => write!(f, "{}: Expected only Tables in '[[bin]]'s, but got {}", path.display(), data_type),
            CargoTomlMissingName{ table, path }             => write!(f, "{}: There is a toplevel '{}' table, but not a nested 'name' field", table, path.display()),
            CargoTomlNameTypeError{ what, path, data_type } => write!(f, "{}: Expected a String as {} name, but got {}", what, path.display(), data_type),

            CargoTomlMissingMembers{ path }              => write!(f, "{}: There is a toplevel '[workspace]' table, but not a nested 'members' list", path.display()),
            CargoTomlMembersTypeError{ path, data_type } => write!(f, "{}: Expected an Array as workspace members, but got {}", path.display(), data_type),
            CargoTomlMemberTypeError{ path, data_type }  => write!(f, "{}: Expected only Strings in workspace members, but got {}", path.display(), data_type),
            CargoTomlPackagesDeduceError{ path }         => write!(f, "{}: No '[package]' or '[workspace]' toplevel table found", path.display()),
        }
    }
}

impl std::error::Error for Error {}





/***** LIBRARY *****/
/// Defines whether to build in release or debug mode.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CargoMode {
    /// Building in release mode.
    Release,
    /// Building in debug/development mode.
    Debug,
}

impl CargoMode {
    /// Converts the CargoMode to a flag.
    #[inline]
    fn to_flag(&self) -> &str {
        use CargoMode::*;
        match self {
            Release => " --release",
            Debug   => "",
        }
    }

    /// Converts the CargoMode to the relevant build folder.
    #[inline]
    fn to_build_dir(&self) -> &str {
        use CargoMode::*;
        match self {
            Releaes => "release",
            Debug   => "debug",
        }
    }



    /* TODO: Default enum functions */
}



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
    /// The build mode (i.e., release or debug) we are in.
    mode     : CargoMode,
}

impl<'a> CargoTarget<'a> {
    /// Constructor for the CargoTarget.
    /// 
    /// Note that the source files and effects of this target will actually be deduced based on the `Cargo.toml` file we assume to be present in the given folder.
    /// 
    /// # Arguments
    /// - `name`: The name/identifier of this target.
    /// - `path`: The path to the directory with the package (or workspace).
    /// - `mode`: Whether to build as release (i.e., with the `--release` flag) or as dev (i.e., without the `--release` flag).
    /// 
    /// # Returns
    /// A new CargoTarget instance.
    /// 
    /// # Errors
    /// This function errors if we failed to deduce packages or effects from the given folder.
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>, mode: CargoMode) -> Result<Self, Error> {
        let name: String  = name.into();
        let path: PathBuf = path.into();

        // Deduce the things
        let effects  : Vec<Box<dyn Effect>> = Self::deduce_effects(&name, &path)?;
        let packages : Vec<String>          = Self::deduce_packages(&name, &path)?;

        // Return the new Target
        Ok(Self {
            name,
            deps : vec![],
            effects,

            path,
            packages,
            mode,
        })
    }



    /// Deduces the list of effects from either the given package or workspace directory by inspecting the Cargo.toml.
    /// 
    /// If the path points to a package, the resulting binary file (either the lib or name) read from the Cargo.toml or deduced is returned.
    /// 
    /// Otherwise, it recursively collects resulting binaries from each package in the workspace.
    /// 
    /// # Arguments
    /// - `name`: The name of the target-to-be (used for debugging purposes only).
    /// - `path`: The path to the directory with the package (or workspace).
    /// 
    /// # Returns
    /// A vector of effects, each of which is the (relevant) output file(s) of a package.
    /// 
    /// # Errors
    /// This function errors if we failed to find, read or parse the `Cargo.toml` file.
    pub fn deduce_effects(name: impl AsRef<str>, path: impl AsRef<Path>) -> Result<Vec<Box<dyn Effect>>, Error> {
        let name : &str  = name.as_ref();
        let path : &Path = path.as_ref();
        trace!("Duducing effects for CargoTarget '{}' in directory '{}'", name, path.display());

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
        debug!("Extracting packages from '{}'...", cargo_path.display());
        if let Value::Table(table) = cargo_toml {
            // If there is a toplevel '[[bin]]', we can deduce the name; otherwise, assume the name
            let names: Vec<String> = if let Some(bins) = table.get("bin") {
                // Assert it is an array
                let bins: &[Value] = match bins {
                    Value::Array(bins) => bins,
                    bins               => { return Err(Error::CargoTomlBinsTypeError{ path: cargo_path, data_type: bins.type_str() }); },  
                };

                // Add all the binaries
                let mut names: Vec<String> = Vec::with_capacity(bins.len());
                for b in bins {
                    // Assert it is a table
                    let bin: &Map<String, Value> = match b {
                        Value::Table(bin) => bin,
                        b                 => { return Err(Error::CargoTomlBinTypeError{ path: cargo_path, data_type: b.type_str() }); },
                    };

                    // Fetch the name field to add it
                    names.push(match bin.get("name") {
                        Some(Value::String(name)) => name.clone(),
                        Some(name)                => { return Err(Error::CargoTomlNameTypeError { what: "bin", path: cargo_path, data_type: name.type_str() }); },
                        None                      => { return Err(Error::CargoTomlMissingName { table: "[bin]", path: cargo_path }); },
                    });
                }
                names

            } else if let Some(package) = table.get("package") {
                // Attempt to find the 'name' field
                match package.get("name") {
                    Some(Value::String(name)) => vec![ name.clone() ],
                    Some(name)                => { return Err(Error::CargoTomlNameTypeError{ what: "package", path: cargo_path, data_type: name.type_str() }); },
                    None                      => { return Err(Error::CargoTomlMissingName{ table: "package", path: cargo_path }); },
                }

            } else {
                return Err(Error::CargoTomlEffectsDeduceError { path: cargo_path });
            };

            // Cast the names to paths
            let paths: Vec<PathBuf> = names.into_iter().map(|n| n.)


            println!("{:?}", names);
            Ok(vec![])
        } else {
            Err(Error::CargoTomlNotATable{ path: cargo_path })
        }
    }

    /// Deduces the list of packages from either the given package or workspace directory by inspecting the Cargo.toml.
    /// 
    /// If the path points to a package, the name of the package is returned as only package.
    /// 
    /// Otherwise, the list of members is returned.
    /// 
    /// # Arguments
    /// - `name`: The name of the target-to-be (used for debugging purposes only).
    /// - `path`: The path to the directory with the package (or workspace).
    /// 
    /// # Returns
    /// A vector of strings, each of which is the name of a package in this directory.
    /// 
    /// # Errors
    /// This function errors if we failed to find, read or parse the `Cargo.toml` file.
    pub fn deduce_packages(name: impl AsRef<str>, path: impl AsRef<Path>) -> Result<Vec<String>, Error> {
        let name : &str  = name.as_ref();
        let path : &Path = path.as_ref();
        trace!("Duducing packages for CargoTarget '{}' in directory '{}'", name, path.display());

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
        debug!("Extracting packages from '{}'...", cargo_path.display());
        if let Value::Table(table) = cargo_toml {
            // Extract either the package itself, the nested packages or both
            let mut res: Vec<String> = vec![];
            if let Some(workspace) = table.get("workspace") {
                // Get the list
                let members: &[Value] = match workspace.get("members") {
                    Some(Value::Array(members)) => members,
                    Some(members)               => { return Err(Error::CargoTomlMembersTypeError { path: cargo_path, data_type: members.type_str() }); },
                    None                        => { return Err(Error::CargoTomlMissingMembers{ path: cargo_path }); },
                };

                // Unwrap the list
                res.reserve(members.len());
                for m in members {
                    res.push(if let Value::String(member) = m {
                        member.clone()
                    } else {
                        return Err(Error::CargoTomlMemberTypeError{ path: cargo_path, data_type: m.type_str() });
                    });
                }

            }
            if let Some(package) = table.get("package") {
                // Attempt to find the 'name' field
                res.push(match package.get("name") {
                    Some(Value::String(name)) => name.clone(),
                    Some(name)                => { return Err(Error::CargoTomlNameTypeError{ what: "package", path: cargo_path, data_type: name.type_str() }); },
                    None                      => { return Err(Error::CargoTomlMissingName{ table: "package", path: cargo_path }); },
                });

            }

            // Return any if we found any
            if !res.is_empty() {
                debug!("Packages deduced from '{}': {:?}", cargo_path.display(), res);
                Ok(res)
            } else { 
                Err(Error::CargoTomlPackagesDeduceError{ path: cargo_path })
            }

        } else {
            Err(Error::CargoTomlNotATable{ path: cargo_path })
        }
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
