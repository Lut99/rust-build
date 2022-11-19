//  MAIN.rs
//    by Lut99
// 
//  Created:
//    16 Nov 2022, 17:57:19
//  Last edited:
//    19 Nov 2022, 11:14:33
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements a very simple installer, that builds the `hello-world` crate
//!   and then copies it to the `/bin` folder.
// 

use std::rc::Rc;

use clap::Parser;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, TermLogger, TerminalMode};

use rust_build::{Cache, Builder, Installer, TargetBuilder};
use rust_build_std::targets::CargoTarget;
use rust_build_std::targets::cargo::CargoMode;


/***** ARGUMENTS *****/
/// Defines the arguments for this installer.
/// 
/// Uses the [clap](https://github.com/clap-rs/clap) library.
#[derive(Parser)]
#[clap(author, about = "Installer for the 'hello-world' app.")]
struct Arguments {
    /// Whether to show trace logs or not.
    #[clap(short, long, help = "If given, shows additional 'trace' logs.")]
    trace : bool,
}





/***** ENTRYPOINT *****/
fn main() {
    // Parse the arguments
    let args: Arguments = Arguments::parse();

    // Setup a logger, just so you can see everything.
    if let Err(err) = TermLogger::init(if args.trace { LevelFilter::Trace } else { LevelFilter::Debug }, Default::default(), TerminalMode::Mixed, ColorChoice::Auto) { eprintln!("WARNING: Failed to setup logger: {} (no logging for this session)", err); }
    info!("Hello World Installer v{}", env!("CARGO_PKG_VERSION"));

    // Define an installer, or at least, the start of it.
    let cache       : Rc<Cache> = Rc::new(Cache::new("./target/make_cache", true).unwrap());
    let mut builder : Builder   = Installer::builder();

    // We have to define so-called _targets_ to build to. This is effectively a single step in the building process.
    // This tutorial requires that we build the `hello-world` crate, which lives in a Cargo workspace. Thus, we can use the `CargoTarget` in the standard library:
    // Note that the creation of the target itself may actually error, to give the target the opportunity to already interact with files.
    let target: CargoTarget = match CargoTarget::builder("hello-world")
        .path("./hello-world")
        .mode(CargoMode::Debug)
        .build(cache.clone())
    {
        Ok(target) => target,
        Err(err)   => { panic!("{}", err); },
    };

    // We can then add the builder
    builder = builder.add_target(target);
}
