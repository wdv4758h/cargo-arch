use toml;
use rustc_serialize;

use std::fs::{self, File};
use std::io::prelude::*;


/// data in Cargo.toml
#[derive(Clone, Debug, RustcDecodable)]
pub struct Cargo {
    pub package: CargoPackage,
}

/// data in [package]
#[derive(Clone, Debug, RustcDecodable)]
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub metadata: CargoMetadata
}

/// data in [package.metadata]
#[derive(Clone, Debug, RustcDecodable)]
pub struct CargoMetadata {
    pub arch: CargoArch
}

/// data in [package.metadata.arch]
#[derive(Clone, Debug, RustcDecodable)]
pub struct CargoArch {
    pub maintainer: Vec<String>,
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub groups: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
    pub provide: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub backup: Vec<String>,
    pub options: Vec<String>,
    pub source: Vec<String>,
    pub md5sum: Vec<String>,
    pub validpgpkeys: Vec<String>,
}

// https://wiki.archlinux.org/index.php/PKGBUILD
#[derive(Debug)]
pub struct ArchConfig {
    pub maintainer: Vec<String>,
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub groups: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
    pub provide: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub backup: Vec<String>,
    pub options: Vec<String>,
    pub source: Vec<String>,
    pub md5sum: Vec<String>,
    pub validpgpkeys: Vec<String>,
}

impl ArchConfig {
    pub fn new() -> ArchConfig {
        let mut content = String::new();
        let path = format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
        let mut path = File::open(path.as_str()).unwrap();
        path.read_to_string(&mut content)
            .expect("cargo-arch: invalid or missing Cargo.toml options");
        toml::decode_str::<Cargo>(&content)
            .expect("cargo-arch: could not decode manifest")
            .to_arch_config()
    }

    pub fn generate_pkgbuild(&self) {
        let mut control = fs::OpenOptions::new().create(true).write(true).open("PKGBUILD").unwrap();
        let mut buffer = String::new();

        macro_rules! add_data {
            ( $fmt: expr, $data: expr ) => {
                buffer.push_str(format!($fmt, $data).as_str());
            }
        }

        fn quote_data(data: &Vec<String>) -> String {
            let mut buffer = String::new();

            if data.len() == 0 {
                return buffer;
            }

            buffer.push_str("\"");
            buffer.push_str(data[0].as_str());
            buffer.push_str("\"");

            for i in data.iter().skip(1) {
                buffer.push_str(", \"");
                buffer.push_str(i);
                buffer.push_str("\"");
            }

            buffer
        }

        for i in &self.maintainer {
            add_data!("# Maintainer: {}\n", i);
        }

        add_data!("pkgname={}\n", self.pkgname);
        add_data!("pkgver={}\n", self.pkgver);
        add_data!("pkgrel={}\n", self.pkgrel);
        add_data!("pkgdesc=\"{}\"\n", self.pkgdesc);
        add_data!("arch=({})\n", quote_data(&self.arch));
        add_data!("url=\"{}\"\n", self.url);
        add_data!("license=({})\n", quote_data(&self.license));
        add_data!("groups=({})\n", quote_data(&self.groups));
        add_data!("depends=({})\n", quote_data(&self.depends));
        add_data!("makedepends=({})\n", quote_data(&self.makedepends));
        add_data!("checkdepends=({})\n", quote_data(&self.checkdepends));
        add_data!("provide=({})\n", quote_data(&self.provide));
        add_data!("conflicts=({})\n", quote_data(&self.conflicts));
        add_data!("replaces=({})\n", quote_data(&self.replaces));
        add_data!("backup=({})\n", quote_data(&self.backup));
        add_data!("options=({})\n", quote_data(&self.options));    // FIXME
        add_data!("source=({})\n", quote_data(&self.source));    // FIXME
        add_data!("md5sum=({})\n", quote_data(&self.md5sum));
        add_data!("validpgpkeys=({})\n", quote_data(&self.validpgpkeys));

        buffer.push_str("\n");
        buffer.push_str(include_str!("PKGBUILD-TEMPLATE"));

        println!("{}", buffer);

        // mksrcinfo
    }
}


impl Cargo {
    fn to_arch_config(&self) -> ArchConfig {
        ArchConfig {
            maintainer: self.package.metadata.arch.maintainer.clone(),
            pkgname: self.package.metadata.arch.pkgname.clone(),
            pkgver: self.package.metadata.arch.pkgver.clone(),
            pkgrel: self.package.metadata.arch.pkgrel.clone(),
            pkgdesc: self.package.metadata.arch.pkgdesc.clone(),
            arch: self.package.metadata.arch.arch.clone(),
            url: self.package.metadata.arch.url.clone(),
            license: self.package.metadata.arch.license.clone(),
            groups: self.package.metadata.arch.groups.clone(),
            depends: self.package.metadata.arch.depends.clone(),
            makedepends: self.package.metadata.arch.makedepends.clone(),
            checkdepends: self.package.metadata.arch.checkdepends.clone(),
            provide: self.package.metadata.arch.provide.clone(),
            conflicts: self.package.metadata.arch.conflicts.clone(),
            replaces: self.package.metadata.arch.replaces.clone(),
            backup: self.package.metadata.arch.backup.clone(),
            options: self.package.metadata.arch.options.clone(),
            source: self.package.metadata.arch.source.clone(),
            md5sum: self.package.metadata.arch.md5sum.clone(),
            validpgpkeys: self.package.metadata.arch.validpgpkeys.clone(),
        }
    }
}
