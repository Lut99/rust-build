//  TESTS.rs
//    by Lut99
// 
//  Created:
//    20 Sep 2022, 22:12:39
//  Last edited:
//    20 Sep 2022, 23:28:00
//  Auto updated?
//    Yes
// 
//  Description:
//!   File that contains tests only, and is used in development to
//!   determine what we want to do.
// 

use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus};

use console::style;


/***** HELPER FUNCTIONS *****/
/// Runs the project in the given tests folder to see if it successfully compiles.
/// 
/// # Arguments
/// - `dir`: The name of the directory that contains the project to test.
/// - `package_name`: The name of the package implementing the installer.
/// - `binary_name`: The name of the binary that is the compiled installer.
/// - `cmd`: A list of commands to give to the installer.
/// 
/// # Returns
/// Nothing. If it does, that means the test succeeded.
/// 
/// # Panics
/// This function panics if the test fails for whatever reason.
fn test_project<S: AsRef<str>>(dir: impl AsRef<str>, package_name: impl AsRef<str>, binary_name: impl AsRef<str>, cmds: Vec<impl AsRef<[S]>>) {
    let dir          : &str = dir.as_ref();
    let package_name : &str = package_name.as_ref();

    // Attempt to find the given folder
    let dir_path: PathBuf = PathBuf::from("./tests").join(dir);
    if !dir_path.exists() { panic!("{}{}", style("Error").red().bold(), style(format!(": No test project directory '{}' found", dir_path.display())).bold()); }
    if !dir_path.is_dir() { panic!("{}{}", style("Error").red().bold(), style(format!(": Test project directory '{}' is not a directory", dir_path.display())).bold()); }

    // Run the cargo command to build the installer in that directory.
    println!("{}", style("Building installer...").bold());
    let mut cmd: Command = Command::new("cargo");
    cmd.arg("build");
    cmd.arg("--package");
    cmd.arg(package_name);
    let mut handle: Child = match cmd.spawn() {
        Ok(handle) => handle,
        Err(err)   => { panic!("{}{}", style("Error").red().bold(), style(format!(": Failed to spawn '{:?}': {}", cmd, err)).bold()); }
    };
    let res: ExitStatus = match handle.wait() {
        Ok(res)  => res,
        Err(err) => { panic!("{}{}", style("Error").red().bold(), style(format!(": Failed to execute command '{:?}': {}", cmd, err)).bold()); },
    };
    if !res.success() { panic!("{}{}", style("Error").red().bold(), style(format!(": Building installer failed with exit code {} (see output above)", res.code().unwrap_or(-1))).bold()); }

    // Now run the installer itself
    for c in cmds.iter().map(|c| c.as_ref().iter().map(|c| c.as_ref()).collect::<Vec<&str>>()) {
        // Prepare the command first
        let mut cmd: Command = Command::new(dir_path.join("target").join("debug").join(binary_name.as_ref()));
        cmd.args(c);

        // Run it
        println!("{}", style(format!("Running '{:?}'...", cmd)).bold());
        let mut handle: Child = match cmd.spawn() {
            Ok(handle) => handle,
            Err(err)   => { panic!("{}{}", style("Error").red().bold(), style(format!(": Failed to spawn '{:?}': {}", cmd, err)).bold()); }
        };
        let res: ExitStatus = match handle.wait() {
            Ok(res)  => res,
            Err(err) => { panic!("{}{}", style("Error").red().bold(), style(format!(": Failed to execute command '{:?}': {}", cmd, err)).bold()); },
        };
        if !res.success() { panic!("{}{}", style("Error").red().bold(), style(format!(": Running '{:?}' failed with exit code {} (see output above)", cmd, res.code().unwrap_or(-1))).bold()); }
    }

    // Success!
}





/***** TESTS *****/
#[test]
fn test_cargo() {
    // Simply compile and then run the builder
    test_project("cargo", "hello-world-installer", "hello-world-installer", vec![
        vec![ "build", "hello-world" ],
    ]);
}
