//! `cargo arch` is a Cargo plugin for making Arch Linux packages.
//! Packages' information is extract from `Cargo.toml`.
//! You can add additional information in `[package.metadata.arch]` section.

use anyhow::{Context, Result};

pub mod cli;
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

    let args = cli::get_args();
    let cli::Commands::Arch(args) = args.command;

    ////////////////////
    // Build Arch Package
    ////////////////////

    build_arch_package(args.mksrcinfo,
                       args.build.unwrap_or(true),
                       args.install,
                       args.syncdeps,
                       args.force,
                       args.manifest_path.as_deref())

}
