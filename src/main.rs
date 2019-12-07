//! `cargo arch` is a Cargo plugin for making Arch Linux packages.
//! Packages' information is extract from `Cargo.toml`.
//! You can add additional information in `[package.metadata.arch]` section.

#[macro_use]
extern crate serde_derive;

use anyhow::{format_err, Context, Result};
use clap::{App, load_yaml};

pub mod config;


fn build_arch_package(mksrcinfo: bool,
                      build: bool,
                      install: bool,
                      syncdeps: bool,
                      force: bool,
                      manifest_path: Option<&str>) -> Result<()> {
    use std::process::Command;
    use std::fs::File;
    use std::io::Write;

    config::ArchConfig::load(manifest_path)?.generate_pkgbuild()?;

    if mksrcinfo {
        let output = Command::new("makepkg")
                             .args(&["--printsrcinfo"])
                             .output()
                             .context("failed to generate .SRCINFO")?;

        let mut file = File::create(".SRCINFO")?;
        file.write_all(&output.stdout)?;
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
                .context("unable to execute makepkg")?
                .wait()
                .context("failed to build package")?;
    }

    Ok(())
}


fn main() -> Result<()> {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("arguments.yml");
    let arguments = App::from_yaml(yml).get_matches();
    let arguments = arguments
        .subcommand_matches("arch")
        .ok_or_else(|| format_err!("arch subcommand not supplied"))?;
    let build = arguments
        .value_of("build")
        .ok_or_else(|| format_err!("no build option"))?
        .parse::<bool>()?;
    let install = arguments.is_present("install");
    let syncdeps = arguments.is_present("syncdeps");
    let force = arguments.is_present("force");
    let mksrcinfo = arguments.is_present("mksrcinfo");
    let manifest_path = arguments.value_of("manifest-path");

    ////////////////////
    // Build Arch Package
    ////////////////////

    build_arch_package(mksrcinfo, build, install, syncdeps, force, manifest_path)

}
