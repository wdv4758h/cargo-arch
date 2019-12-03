//! Arch Linux's package config

use std::fs::File;
use std::io::prelude::*;

use toml;

use super::core::{Cargo, ToPackageConfig, GeneratePackageConfig};


/// default arch in Arch Linux is x86_64
fn default_arch() -> Vec<String> {
    vec!["x86_64".to_string()]
}

fn empty_string() -> String {
    "".to_string()
}

/// data in `[package.metadata.arch]` section
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CargoArch {
    /// The maintainers of the package
    pub maintainers: Option<Vec<String>>,
    /// The name of the package.
    pub pkgname: Option<String>,
    /// The version of the software as released from the author.
    pub pkgver: Option<String>,
    /// This is the release number specific to the Arch Linux release.
    pub pkgrel: Option<String>,
    /// Used to force the package to be seen as newer than any previous versions with a lower epoch,
    /// even if the version number would normally not trigger such an upgrade.
    pub epoch: Option<String>,
    /// This should be a brief description of the package and its functionality.
    pub pkgdesc: Option<String>,
    /// This field contains a URL that is associated with the software being packaged.
    /// This is typically the project’s web site.
    pub url: Option<String>,
    /// This field specifies the license(s) that apply to the package.
    pub license: Option<Vec<String>>,
    /// Specifies a special install script that is to be included in the package.
    #[serde(default = "empty_string")]
    pub install: String,
    /// Specifies a changelog file that is to be included in the package.
    #[serde(default = "empty_string")]
    pub changelog: String,
    /// An array of source files required to build the package.
    #[serde(default)]
    pub source: Vec<String>,
    /// An array of PGP fingerprints.
    #[serde(default)]
    pub validpgpkeys: Vec<String>,
    /// An array of file names corresponding to those from the source array.
    #[serde(default)]
    pub noextract: Vec<String>,
    /// This array contains an MD5 hash for every source file specified in the source array (in the same order).
    #[serde(default)]
    pub md5sums: Vec<String>,
    /// Alternative integrity checks that makepkg supports; these all behave similar to the md5sums option described above.
    #[serde(default)]
    pub sha1sums: Vec<String>,
    /// Alternative integrity checks that makepkg supports; these all behave similar to the md5sums option described above.
    #[serde(default)]
    pub sha256sums: Vec<String>,
    /// Alternative integrity checks that makepkg supports; these all behave similar to the md5sums option described above.
    #[serde(default)]
    pub sha384sums: Vec<String>,
    /// Alternative integrity checks that makepkg supports; these all behave similar to the md5sums option described above.
    #[serde(default)]
    pub sha512sums: Vec<String>,
    /// An array of symbolic names that represent groups of packages,
    /// allowing you to install multiple packages by requesting a single target.
    #[serde(default)]
    pub groups: Vec<String>,
    /// Defines on which architectures the given package is available.
    #[serde(default = "default_arch")]
    pub arch: Vec<String>,
    /// An array of file names, without preceding slashes,
    /// that should be backed up if the package is removed or upgraded.
    #[serde(default)]
    pub backup: Vec<String>,
    /// An array of packages this package depends on to run.
    #[serde(default)]
    pub depends: Vec<String>,
    /// An array of packages this package depends on to build but are not needed at runtime.
    #[serde(default)]
    pub makedepends: Vec<String>,
    /// An array of packages this package depends on to run its test suite but are not needed at runtime.
    #[serde(default)]
    pub checkdepends: Vec<String>,
    /// An array of packages (and accompanying reasons) that are not essential for base functionality,
    /// but may be necessary to make full use of the contents of this package.
    #[serde(default)]
    pub optdepends: Vec<String>,
    /// An array of packages that will conflict with this package.
    #[serde(default)]
    pub conflicts: Vec<String>,
    /// An array of "virtual provisions" this package provides.
    #[serde(default)]
    pub provides: Vec<String>,
    /// An array of packages this package should replace.
    #[serde(default)]
    pub replaces: Vec<String>,
    /// This array allows you to override some of makepkg’s default behavior when building packages.
    #[serde(default)]
    pub options: Vec<String>,
}

/// see `man PKGBUILD`
/// and https://wiki.archlinux.org/index.php/PKGBUILD
#[derive(Debug)]
pub struct ArchConfig {
    /// The maintainers of the package
    pub maintainers: Vec<String>,
    /// The name of the package.
    pub pkgname: String,
    /// The version of the software as released from the author.
    pub pkgver: String,
    /// This is the release number specific to the Arch Linux release.
    pub pkgrel: String,
    /// Used to force the package to be seen as newer than any previous versions with a lower epoch,
    /// even if the version number would normally not trigger such an upgrade.
    pub epoch: String,
    /// This should be a brief description of the package and its functionality.
    pub pkgdesc: String,
    /// This field contains a URL that is associated with the software being packaged.
    /// This is typically the project’s web site.
    pub url: String,
    /// This field specifies the license(s) that apply to the package.
    pub license: Vec<String>,
    /// Specifies a special install script that is to be included in the package.
    pub install: String,
    /// Specifies a changelog file that is to be included in the package.
    pub changelog: String,
    /// An array of source files required to build the package.
    pub source: Vec<String>,
    /// An array of PGP fingerprints.
    pub validpgpkeys: Vec<String>,
    /// An array of file names corresponding to those from the source array.
    pub noextract: Vec<String>,
    /// This array contains an MD5 hash for every source file specified in the source array (in the same order).
    pub md5sums: Vec<String>,
    /// Alternative integrity checks that makepkg supports; these all behave similar to the md5sums option described above.
    pub sha1sums: Vec<String>,
    /// Alternative integrity checks that makepkg supports; these all behave similar to the md5sums option described above.
    pub sha256sums: Vec<String>,
    /// Alternative integrity checks that makepkg supports; these all behave similar to the md5sums option described above.
    pub sha384sums: Vec<String>,
    /// Alternative integrity checks that makepkg supports; these all behave similar to the md5sums option described above.
    pub sha512sums: Vec<String>,
    /// An array of symbolic names that represent groups of packages,
    /// allowing you to install multiple packages by requesting a single target.
    pub groups: Vec<String>,
    /// Defines on which architectures the given package is available.
    pub arch: Vec<String>,
    /// An array of file names, without preceding slashes,
    /// that should be backed up if the package is removed or upgraded.
    pub backup: Vec<String>,
    /// An array of packages this package depends on to run.
    pub depends: Vec<String>,
    /// An array of packages this package depends on to build but are not needed at runtime.
    pub makedepends: Vec<String>,
    /// An array of packages this package depends on to run its test suite but are not needed at runtime.
    pub checkdepends: Vec<String>,
    /// An array of packages (and accompanying reasons) that are not essential for base functionality,
    /// but may be necessary to make full use of the contents of this package.
    pub optdepends: Vec<String>,
    /// An array of packages that will conflict with this package.
    pub conflicts: Vec<String>,
    /// An array of "virtual provisions" this package provides.
    pub provides: Vec<String>,
    /// An array of packages this package should replace.
    pub replaces: Vec<String>,
    /// This array allows you to override some of makepkg’s default behavior when building packages.
    pub options: Vec<String>,
}

impl ArchConfig {
    pub fn new(manifest_path: Option<&str>) -> ArchConfig {
        let mut content = String::new();
        let path = format!(
            "{}/Cargo.toml",
            match manifest_path {
                Some(val) => val.to_string(),
                None => match std::env::var("CARGO_MANIFEST_DIR") {
                    Ok(val) => val,
                    Err(_) => ".".to_string(),
                }
            }
        );
        let mut path = File::open(path.as_str()).unwrap();
        path.read_to_string(&mut content)
            .expect("cargo-arch: invalid or missing Cargo.toml options");
        toml::from_str::<Cargo>(&content)
            .expect("cargo-arch: could not decode manifest")
            .to_config()
    }

    pub fn generate_pkgbuild(&self) {
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

        for i in &self.maintainers {
            add_data!("# Maintainer: {}\n", i);
        }
        buffer.push_str("\n");

        add_data!("pkgname={}\n", self.pkgname);
        add_data!("pkgver={}\n", self.pkgver.replace("-","_"));
        add_data!("pkgrel={}\n", self.pkgrel);
        add_data!("epoch={}\n", self.epoch);
        add_data!("pkgdesc=\"{}\"\n", self.pkgdesc);
        add_data!("url=\"{}\"\n", self.url);
        add_data!("license=({})\n", quote_data(&self.license));
        add_data!("install=\"{}\"\n", self.install);
        add_data!("changelog=\"{}\"\n", self.changelog);
        add_data!("source=({})\n", quote_data(&self.source));
        add_data!("validpgpkeys=({})\n", quote_data(&self.validpgpkeys));
        add_data!("noextract=({})\n", quote_data(&self.noextract));
        add_data!("md5sums=({})\n", quote_data(&self.md5sums));
        add_data!("sha1sums=({})\n", quote_data(&self.sha1sums));
        add_data!("sha256sums=({})\n", quote_data(&self.sha256sums));
        add_data!("sha384sums=({})\n", quote_data(&self.sha384sums));
        add_data!("sha512sums=({})\n", quote_data(&self.sha512sums));
        add_data!("groups=({})\n", quote_data(&self.groups));
        add_data!("arch=({})\n", quote_data(&self.arch));
        add_data!("backup=({})\n", quote_data(&self.backup));
        add_data!("depends=({})\n", quote_data(&self.depends));
        add_data!("makedepends=({})\n", quote_data(&self.makedepends));
        add_data!("checkdepends=({})\n", quote_data(&self.checkdepends));
        add_data!("optdepends=({})\n", quote_data(&self.optdepends));
        add_data!("conflicts=({})\n", quote_data(&self.conflicts));
        add_data!("provides=({})\n", quote_data(&self.provides));
        add_data!("replaces=({})\n", quote_data(&self.replaces));
        add_data!("options=({})\n", quote_data(&self.options));

        buffer.push_str("\n");
        buffer.push_str(include_str!("PKGBUILD-TEMPLATE"));

        let mut file = File::create("PKGBUILD").unwrap();
        write!(file, "{}", buffer).unwrap();
    }
}

impl ToPackageConfig<ArchConfig> for Cargo {
    fn to_config(&self) -> ArchConfig {
        let arch_config = &self.package.metadata.arch;

        let maintainers = arch_config.maintainers.as_ref().unwrap_or(&self.package.authors).clone();
        let pkgname = arch_config.pkgname.as_ref().unwrap_or(&self.package.name).clone();
        let pkgver = arch_config.pkgver.as_ref().unwrap_or(&self.package.version).clone();
        let pkgrel = arch_config.pkgrel.as_ref().unwrap_or(&"1".to_string()).clone();
        let epoch = arch_config.epoch.as_ref().unwrap_or(&"0".to_string()).clone();
        let pkgdesc = arch_config.pkgdesc.as_ref().unwrap_or(&self.package.description).clone();
        let url = arch_config.url.as_ref()
                             .or(self.package.homepage.as_ref())
                             .or(self.package.repository.as_ref())
                             .unwrap_or(&String::new())
                             .clone();
        let license = arch_config.license.as_ref().unwrap_or(
            &self.package.license.split("/")
                                 .map(|s| s.to_string())
                                 .collect::<Vec<String>>()
        ).clone();

        ArchConfig {
            maintainers: maintainers,
            pkgname: pkgname,
            pkgver: pkgver,
            pkgrel: pkgrel,
            epoch: epoch,
            pkgdesc: pkgdesc,
            url: url,
            license: license,
            install: arch_config.install.to_string(),
            changelog: arch_config.changelog.to_string(),
            source: arch_config.source.to_vec(),
            validpgpkeys: arch_config.validpgpkeys.to_vec(),
            noextract: arch_config.noextract.to_vec(),
            md5sums: arch_config.md5sums.to_vec(),
            sha1sums: arch_config.sha1sums.to_vec(),
            sha256sums: arch_config.sha256sums.to_vec(),
            sha384sums: arch_config.sha384sums.to_vec(),
            sha512sums: arch_config.sha512sums.to_vec(),
            groups: arch_config.groups.to_vec(),
            arch: arch_config.arch.to_vec(),
            backup: arch_config.backup.to_vec(),
            depends: arch_config.depends.to_vec(),
            makedepends: arch_config.makedepends.to_vec(),
            checkdepends: arch_config.checkdepends.to_vec(),
            optdepends: arch_config.optdepends.to_vec(),
            conflicts: arch_config.conflicts.to_vec(),
            provides: arch_config.provides.to_vec(),
            replaces: arch_config.replaces.to_vec(),
            options: arch_config.options.to_vec(),
        }
    }
}


impl GeneratePackageConfig for ArchConfig {
    fn generate_package_config(&self) {
        self.generate_pkgbuild();
    }
}
