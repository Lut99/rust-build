//  SHELL.rs
//    by Lut99
// 
//  Created:
//    19 Nov 2022, 12:09:33
//  Last edited:
//    19 Nov 2022, 12:29:31
//  Auto updated?
//    Yes
// 
//  Description:
//!   Contains higher-level wrappers around `std` commands to make CLI
//!   interaction easier.
// 

use std::collections::HashMap;

pub use crate::errors::ShellCommandError as Error;


/***** LIBRARY *****/
/// Defines a shell command that can be run when building.
#[derive(Clone, Debug)]
pub struct ShellCommand {
    /// The executable to run.
    exec : String,
    /// The arguments to pass to the executable.
    args : Vec<String>,
    /// Additional environment variables to set.
    envs : HashMap<String, String>,
}

impl ShellCommand {
    /// Constructor for the ShellCommand that initializes it without any arguments or environment variables set.
    /// 
    /// # Arguments
    /// - `exec`: The executable to run.
    /// 
    /// # Returns
    /// A new ShellCommand for the executable only.
    #[inline]
    pub fn exec_only(exec: impl Into<String>) -> Self {
        Self {
            exec : exec.into(),
            args : vec![],
            envs : HashMap::new(),
        }
    }

    /// Constructor for the ShellCommand that initializes it with the given arguments (but not yet any environment variables).
    /// 
    /// # Arguments
    /// - `exec`: The executable to run.
    /// - `args`: An iterator that produces the arguments to set.
    /// 
    /// # Returns
    /// A new ShellCommand for the executable with (an initial set of) arguments.
    #[inline]
    pub fn with_args(exec: impl Into<String>, args: impl IntoIterator<Item = impl Into<String>, IntoIter = impl Iterator<Item = impl Into<String>>>) -> Self {
        Self {
            exec : exec.into(),
            args : args.into_iter().map(|a| a.into()).collect(),
            envs : HashMap::new(),
        }
    }

    /// Constructor for the ShellCommand that initializes it with the given environment variables (but not yet any arguments).
    /// 
    /// # Arguments
    /// - `exec`: The executable to run.
    /// - `envs`: An iterator that produces pairs of (name, value) for the environment variables to add.
    /// 
    /// # Returns
    /// A new ShellCommand for the executable with (an initial set of) environment variables.
    #[inline]
    pub fn with_envs(exec: impl Into<String>, envs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>), IntoIter = impl Iterator<Item = (impl Into<String>, impl Into<String>)>>) -> Self {
        Self {
            exec : exec.into(),
            args : vec![],
            envs : envs.into_iter().map(|(n, v)| (n.into(), v.into())).collect(),
        }
    }

    /// Constructor for the ShellCommand that initializes it with the given arguments and environment variables.
    /// 
    /// # Arguments
    /// - `exec`: The executable to run.
    /// - `args`: An iterator that produces the arguments to set.
    /// - `envs`: An iterator that produces pairs of (name, value) for the environment variables to add.
    /// 
    /// # Returns
    /// A new ShellCommand for the executable with (an initial set of) arguments and environment variables.
    #[inline]
    pub fn new(exec: impl Into<String>, args: impl IntoIterator<Item = impl Into<String>, IntoIter = impl Iterator<Item = impl Into<String>>>, envs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>), IntoIter = impl Iterator<Item = (impl Into<String>, impl Into<String>)>>) -> Self {
        Self {
            exec : exec.into(),
            args : args.into_iter().map(|a| a.into()).collect(),
            envs : envs.into_iter().map(|(n, v)| (n.into(), v.into())).collect(),
        }
    }



    /// Adds a new argument to this ShellCommand.
    /// 
    /// # Arguments
    /// - `arg`: The new argument to add.
    #[inline]
    pub fn add_arg(&mut self, arg: impl Into<String>) {
        self.args.push(arg.into());
    }
    /// Adds a collection of new arguments to this ShellCommand.
    /// 
    /// # Arguments
    /// - `args`: An iterator that produces the new arguments to add.
    #[inline]
    pub fn add_args(&mut self, args: impl IntoIterator<Item = impl Into<String>, IntoIter = impl Iterator<Item = impl Into<String>>>) {
        let mut args: Vec<String> = args.into_iter().map(|a| a.into()).collect();
        self.args.append(&mut args);
    }

    /// Sets a new environment variable for this ShellCommand.
    /// 
    /// # Arguments
    /// - `name`: The name of the environment variable to add.
    /// - `value`: The value of the environment variable to add.
    #[inline]
    pub fn add_env(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.envs.insert(name.into(), value.into());
    }
    /// Sets a collection of new environment variables for this ShellCommand.
    /// 
    /// # Arguments
    /// - `envs`: An iterator that produces the new environment variables (as (name, value) tuples) to add.
    #[inline]
    pub fn add_envs(&mut self, envs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>), IntoIter = impl Iterator<Item = (impl Into<String>, impl Into<String>)>>) {
        let envs: HashMap<String, String> = envs.into_iter().map(|(n, v)| (n.into(), v.into())).collect();
        self.envs.extend(envs);
    }



    /// Runs the command that is build in this ShellCommand.
    /// 
    /// This variation does not return anything from the underlying command - only its return code.
    /// 
    /// # Returns
    /// The return code of the command once it completes.
    /// 
    /// # Errors
    /// This function may fail if we failed to even launch the executable in the first place.
    #[inline]
    pub fn run(&self) -> Result<i32, Error> {
        
    }
}
