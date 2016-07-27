#![feature(custom_derive)]

#[macro_use]
extern crate clap;
extern crate toml;
extern crate rustc_serialize;

use clap::App;

use std::process::Command;
use std::fs::File;
use std::io::Write;

pub mod config;


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("arguments.yml");
    let arguments = App::from_yaml(yml).get_matches();
    let arguments = arguments.subcommand_matches("arch").unwrap();
    let build = arguments.value_of("build").unwrap();
    let install = arguments.is_present("install");
    let syncdeps = arguments.is_present("syncdeps");
    let force = arguments.is_present("force");
    let mksrcinfo = arguments.is_present("mksrcinfo");

    ////////////////////
    // Generate PKGBUILD
    ////////////////////

    let config = config::ArchConfig::new();
    config.generate_pkgbuild();

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

    if build == "true" {
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
