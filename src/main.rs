#![feature(custom_derive)]

extern crate toml;
extern crate rustc_serialize;

mod config;


fn main() {
    let config = config::ArchConfig::new();
    config.generate_pkgbuild();
}
