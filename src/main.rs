//! `cargo arch` is a Cargo plugin for making Arch Linux packages.
//! Packages' information is extract from `Cargo.toml`.
//! You can add additional information in `[package.metadata.arch]` section.

#[macro_use]
extern crate serde_derive;

use clap::{App, load_yaml};

pub mod config;


fn build_arch_package(mksrcinfo: bool,
                      build: bool,
                      install: bool,
                      syncdeps: bool,
                      force: bool) {
    use std::process::Command;
    use std::fs::File;
    use std::io::Write;
    use crate::config::core::GeneratePackageConfig;

    config::ArchConfig::new().generate_package_config();

    if mksrcinfo {
        let output = Command::new("makepkg")
                             .args(&["--printsrcinfo"])
                             .output()
                             .expect("failed to generate .SRCINFO");

        let mut file = File::create(".SRCINFO").unwrap();
        file.write_all(&output.stdout).unwrap();
    }

    ////////////////////
    // Build Package
    ////////////////////

    if build {
        let mut args = vec![];

        if install {
            args.push("--install");
        }
        if syncdeps {
            args.push("--syncdeps");
        }
        if force {
            args.push("--force");
        }

        Command::new("makepkg")
                .args(&args)
                .spawn()
                .unwrap()
                .wait()
                .expect("failed to build package");
    }
}


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("arguments.yml");
    let arguments = App::from_yaml(yml).get_matches();
    let arguments = arguments.subcommand_matches("arch").unwrap();
    let build = arguments.value_of("build").unwrap().parse::<bool>().unwrap();
    let install = arguments.is_present("install");
    let syncdeps = arguments.is_present("syncdeps");
    let force = arguments.is_present("force");
    let mksrcinfo = arguments.is_present("mksrcinfo");

    ////////////////////
    // Build Arch Package
    ////////////////////

    build_arch_package(mksrcinfo, build, install, syncdeps, force);

}
