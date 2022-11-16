//  MAIN.rs
//    by Lut99
// 
//  Created:
//    16 Nov 2022, 17:57:19
//  Last edited:
//    16 Nov 2022, 18:15:11
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements a very simple installer, that builds the `hello-world` crate
//!   and then copies it to the `/bin` folder.
// 

use clap::Parser;

use rust_build::{Builder, Installer};
use rust_build_std::targets::CargoTarget;


/***** ARGUMENTS *****/
/// Defines the arguments for this installer.
/// 
/// Uses the [clap](https://github.com/clap-rs/clap) library.
#[derive(Parser)]
#[clap(author, about = "Installer for the 'hello-world' app.")]
struct Arguments {
    
}





/***** ENTRYPOINT *****/
fn main() {
    // Setup a logger, just so you can see everything.

    // Define an installer, or at least, the start of it.
    let mut builder: Builder = Installer::builder();

    // We have to define so-called _targets_ to build to. This is effectively a single step in the building process.
    // This tutorial requires that we build the `hello-world` crate, which lives in a Cargo workspace. Thus, we can use the `CargoTarget` in the standard library:
    // Note that the creation of the target itself may actually error, to give the target the opportunity to already interact with files.
    let target: CargoTarget = match CargoTarget::new(
        // The name of a target defines how users (and we!) may refer to it, and must be unique across all targets.
        "hello-world",
        // The path is the path to the workspace or package that we build. We can give the path relative to the `context`, which is the current working directory of the user by default.
        "./hello-world",

        // There are other options, but for now this is enough
    ) {
        Ok(target) => target,
        Err(err)   => { panic!("{}", err); },
    };

    // We can then add the builder
    builder = builder.add_target(target);
}