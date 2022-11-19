//  CARGO.rs
//    by Lut99
// 
//  Created:
//    13 Nov 2022, 14:34:33
//  Last edited:
//    19 Nov 2022, 12:09:02
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
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use toml::Value;
use toml::map::Map;

use rust_build::errors::TargetError;
use rust_build::spec::{Architecture, Effect, Named, OperatingSystem, Target, TargetBuilder};
use rust_build::view::EffectView;
use rust_build::cache::Cache;

use crate::{debug, trace};
use crate::effects::File;


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

            CargoTomlEffectsDeduceError{ path }             => write!(f, "{}: No '[[bin]]', '[package]' or '[workspace]' toplevel table found", path.display()),
            CargoTomlBinsTypeError{ path, data_type }       => write!(f, "{}: Expected an Array as '[[bin]]'s, but got {}", path.display(), data_type),
            CargoTomlBinTypeError{ path, data_type }        => write!(f, "{}: Expected only Tables in '[[bin]]'s, but got {}", path.display(), data_type),
            CargoTomlMissingName{ table, path }             => write!(f, "{}: There is a toplevel '{}' table, but not a nested 'name' field", table, path.display()),
            CargoTomlNameTypeError{ what, path, data_type } => write!(f, "{}: Expected a String as {} name, but got {}", what, path.display(), data_type),
            CargoTomlMissingMembers{ path }                 => write!(f, "{}: There is a toplevel '[workspace]' table, but not a nested 'members' list", path.display()),
            CargoTomlMembersTypeError{ path, data_type }    => write!(f, "{}: Expected an Array as workspace members, but got {}", path.display(), data_type),
            CargoTomlMemberTypeError{ path, data_type }     => write!(f, "{}: Expected only Strings in workspace members, but got {}", path.display(), data_type),
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
    pub fn to_flag(&self) -> &str {
        use CargoMode::*;
        match self {
            Release => " --release",
            Debug   => "",
        }
    }

    /// Converts the CargoMode to the relevant build folder.
    #[inline]
    pub fn to_build_dir(&self) -> &str {
        use CargoMode::*;
        match self {
            Release => "release",
            Debug   => "debug",
        }
    }
}



/// Defines the builder for the `CargoTarget`.
/// 
/// Note that you have to call at least `CargoTargetBuilder::path()` before calling `CargoTargetBuilder::build()`.
/// 
/// Also note that if you do not specify any effects, they will automatically be deduced from the `Cargo.toml` file(s) sa all binaries they produce.
pub struct CargoTargetBuilder<'a> {
    /// The name of this target.
    name    : String,
    /// The dependencies of this target.
    deps    : Vec<EffectView<'a>>,
    /// The effects (that we care about) of this target.
    effects : Option<Vec<Box<dyn Effect>>>,

    /// The path of the directory where the target package (or workspace) lives.
    path     : Option<PathBuf>,
    /// The packages that we build in this run.
    packages : Vec<String>,
    /// The build mode (i.e., release or debug) we are in.
    mode     : CargoMode,
}

impl<'a> TargetBuilder<'a> for CargoTargetBuilder<'a> {
    type Target = CargoTarget<'a>;


    #[inline]
    fn new(name: impl Into<String>) -> Self {
        Self {
            name    : name.into(),
            deps    : vec![],
            effects : None,

            path     : None,
            packages : vec![],
            mode     : CargoMode::Release,
        }
    }



    #[inline]
    fn dep(mut self, dep: EffectView<'a>) -> Self {
        self.deps.push(dep);
        self
    }
    #[inline]
    fn deps(mut self, deps: impl IntoIterator<Item = EffectView<'a>, IntoIter = impl Iterator<Item = EffectView<'a>>>) -> Self {
        // Collect them in a separate vector first
        let mut new_deps: Vec<EffectView> = deps.into_iter().collect();
        self.deps.append(&mut new_deps);
        self
    }

    fn effect(mut self, effect: impl 'static + Effect) -> Self {
        // Either set or add
        if let Some(effects) = &mut self.effects {
            effects.push(Box::new(effect));
        } else {
            self.effects = Some(vec![ Box::new(effect) ]);
        }
        self
    }
    fn effects(mut self, effects: impl IntoIterator<Item = impl 'static + Effect, IntoIter = impl Iterator<Item = impl 'static + Effect>>) -> Self {
        // Collect them in a separate vector first
        let mut new_effects: Vec<Box<dyn Effect>> = effects.into_iter().map(|e| Box::new(e) as Box<dyn Effect>).collect();

        // Either set or add
        if let Some(effects) = &mut self.effects {
            effects.append(&mut new_effects);
        } else {
            self.effects = Some(new_effects);
        }
        self
    }



    fn build(self, cache: Rc<Cache>) -> Result<Self::Target, Box<dyn std::error::Error>> {
        // Assert we have what we need and/or default
        let path: PathBuf = match self.path {
            Some(path) => path,
            None       => { panic!("You have to call `CargoTargetBuilder::path()` before callign `CargoTargetBuilder::build()`"); },
        };
        let effects: Vec<Box<dyn Effect>> = match self.effects {
            Some(effects) => effects,
            None          => { CargoTarget::deduce_effects(&self.name, &path, self.mode, cache).map_err(|err| Box::new(err))? },
        };

        // Simply create a target with those properties
        Ok(CargoTarget {
            name : self.name,
            deps : self.deps,
            effects,

            path,
            packages : self.packages,
            mode     : self.mode,
        })
    }
}

impl<'a> CargoTargetBuilder<'a> {
    /// Sets the path of the directory that this CargoTargetBuilder operates in.
    /// 
    /// This function is mandatory to set before calling `CargoTargetBuilder::build()`.
    /// 
    /// # Arguments
    /// - `path`: The path to the package or workspace directory that this target concerns itself with.
    /// 
    /// # Returns
    /// The same `self` as given for chaining purposes.
    #[inline]
    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Adds a package to the list of packages that this target will build.
    /// 
    /// If you specify no packages at all (i.e., never call `CargoTargetBuilder::package()` and `CargoTargetBuilder::packages()`), then all packages in a directory will be built (akin to not specifying any packages when calling `cargo build`).
    /// 
    /// # Arguments
    /// - `package`: The name/identifier of the package to build.
    /// 
    /// # Returns
    /// The same `self` as given for chaining purposes.
    #[inline]
    pub fn package(mut self, package: impl Into<String>) -> Self {
        self.packages.push(package.into());
        self
    }
    /// Adds a whole list of packages to the list of packages that this target will build.
    /// 
    /// If you specify no packages at all (i.e., never call `CargoTargetBuilder::package()` and `CargoTargetBuilder::packages()`), then all packages in a directory will be built (akin to not specifying any packages when calling `cargo build`).
    /// 
    /// # Arguments
    /// - `packages`: An iterator over the names/identifiers of the packages to build.
    /// 
    /// # Returns
    /// The same `self` as given for chaining purposes.
    #[inline]
    pub fn packages(mut self, packages: impl IntoIterator<Item = impl Into<String>, IntoIter = impl Iterator<Item = impl Into<String>>>) -> Self {
        // Collect them in a separate vector first
        let mut new_packages: Vec<String> = packages.into_iter().map(|p| p.into()).collect();
        self.packages.append(&mut new_packages);
        self
    }

    /// Sets the building mode for this target.
    /// 
    /// Defaults to `CargoMode::Release`.
    /// 
    /// # Arguments
    /// - `mode`: The mode in which to build the packages.
    /// 
    /// # Returns
    /// The same `self` as given for chaining purposes.
    #[inline]
    pub fn mode(mut self, mode: CargoMode) -> Self {
        self.mode = mode;
        self
    }
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
    /// Returns a builder for the CargoTarget that can be used to fully define it.
    /// 
    /// Note that you have to call at least `CargoTargetBuilder::path()` before calling `CargoTargetBuilder::build()`.
    /// 
    /// Also note that if you do not specify any effects, they will automatically be deduced from the `Cargo.toml` file(s) sa all binaries they produce.
    /// 
    /// # Arguments
    /// - `name`: The name of the target to build.
    /// 
    /// # Returns
    /// A new CargoTargetBuilder instance.
    #[inline]
    pub fn builder(name: impl Into<String>) -> CargoTargetBuilder<'a> {
        CargoTargetBuilder::new(name)
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
    /// - `mode`: Whether we're building in `CargoMode::Release` or `CargoMode::Debug` mode.
    /// - `cache`: The Cache that we use to keep track of file changed.
    /// 
    /// # Returns
    /// A vector of effects, each of which is the (relevant) output file(s) of a package.
    /// 
    /// # Errors
    /// This function errors if we failed to find, read or parse the `Cargo.toml` file.
    pub fn deduce_effects(name: impl AsRef<str>, path: impl AsRef<Path>, mode: CargoMode, cache: Rc<Cache>) -> Result<Vec<Box<dyn Effect>>, Error> {
        let name : &str  = name.as_ref();
        let path : &Path = path.as_ref();
        trace!("Duducing effects for CargoTarget '{}' in directory '{}'", name, path.display());

        // Attempt to open the Cargo.toml file and read its contents
        let cargo_path: PathBuf = path.join("Cargo.toml");
        let cargo_toml: Vec<u8> = match fs::File::open(&cargo_path) {
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
        debug!("Extracting effects from '{}'...", cargo_path.display());
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
                vec![]
            };

            // Cast the names to paths, then to (File) effects
            let mut res: Vec<Box<dyn Effect>> = names.into_iter().map(|n| {
                // First, create a path from that
                let path: PathBuf = PathBuf::from("./target").join(mode.to_build_dir()).join(&n);

                // Next, wrap it in a FileEffect
                Box::new(File::new(format!("{}_{}", name, n), cache.clone(), path)) as Box<dyn Effect>
            }).collect();

            // Recurse into any workspace files to handle those
            if let Some(workspace) = table.get("workspace") {
                // Get the list
                let members: &[Value] = match workspace.get("members") {
                    Some(Value::Array(members)) => members,
                    Some(members)               => { return Err(Error::CargoTomlMembersTypeError{ path: cargo_path, data_type: members.type_str() }); },
                    None                        => { return Err(Error::CargoTomlMissingMembers{ path: cargo_path }); },
                };

                // Unwrap the list to strings
                let mut smembers: Vec<&String> = Vec::with_capacity(members.len());
                for m in members {
                    smembers.push(if let Value::String(member) = m {
                        member
                    } else {
                        return Err(Error::CargoTomlMemberTypeError{ path: cargo_path, data_type: m.type_str() });
                    });
                }

                // We can now recurse each of the members to find their package names
                for m in smembers {
                    res.append(&mut Self::deduce_effects(name, path.join(m), mode, cache.clone())?);
                }
            }

            // If the names are still empty, we failed
            if res.is_empty() {
                return Err(Error::CargoTomlEffectsDeduceError { path: cargo_path });
            };

            // Return that
            debug!("Effects deduced from '{}': {:?}", cargo_path.display(), res.iter().map(|e| e.name()).collect::<Vec<&str>>());
            Ok(res)
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

    /// Returns the mode in which we're building.
    #[inline]
    pub fn mode(&self) -> CargoMode { self.mode }
}

impl<'a> Named for CargoTarget<'a> {
    #[inline]
    fn name(&self) -> &str { &self.name }
}
impl<'a> Target for CargoTarget<'a> {
    fn build(&self, os: OperatingSystem, arch: Architecture, dry_run: bool) -> Result<(), TargetError> {
        // Cast architectures to a suitable string
        let arch: &str = match arch {
            Architecture::x86_32       => "i686",
            Architecture::x86_64       => "x86_64",
            Architecture::Aarch32      => "arm",
            Architecture::Aarch64      => "aarch64",
            Architecture::PowerPc32    => "powerpc",
            Architecture::PowerPc64    => "powerpc64",
            Architecture::Mips         => "mips",
            Architecture::Custom(arch) => { panic!("Custom architectures ('{}') are not supported by CargoTarget", arch); },
        };

        // Use that to prepare the cargo target string
        let target: String = match os {
            OperatingSystem::Windows      => { format!("{}-pc-windows-msvc", arch) },
            OperatingSystem::MacOs        => { format!("{}-apple-darwin", arch) },
            OperatingSystem::Linux        => { format!("{}-unknown-linux-gnu", arch) },
            OperatingSystem::Custom(arch) => { panic!("Custom operating systems ('{}') are not supported by CargoTarget", arch); },
        };

        // Now prepare the command to run
        

        Ok(())
    }



    #[inline]
    fn deps(&self) -> &[EffectView] { &self.deps }

    #[inline]
    fn effects(&self) -> &[Box<dyn Effect>] { &self.effects }
}
