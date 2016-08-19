//! `cargo arch` is a Cargo plugin for making Arch Linux packages.
//! Packages' information is extract from `Cargo.toml`.
//! You can add additional information in `[package.metadata.arch]` section.

#![feature(custom_derive)]

#[macro_use]
extern crate clap;
extern crate toml;
extern crate rustc_serialize;

use clap::App;

pub mod config;
pub mod utils;

pub use utils::*;


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
