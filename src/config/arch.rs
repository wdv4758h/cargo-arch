//! Arch Linux's package config

use crate::config::core;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;

use toml;

use super::core::Cargo;

/// default arch in Arch Linux is x86_64
fn default_arch() -> Vec<String> {
    vec!["x86_64".to_string()]
}

fn empty_string() -> String {
    "".to_string()
}

/// cargo is used by the template so default that
fn default_depends() -> Vec<String> {
    vec!["cargo".to_string()]
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
    #[serde(default)]
    pub epoch: u64,
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
    #[serde(default = "default_depends")]
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
    pub epoch: u64,
    /// This should be a brief description of the package and its functionality.
    pub pkgdesc: String,
    /// This field contains a URL that is associated with the software being packaged.
    /// This is typically the project’s web site.
    pub url: String,
    /// This field Represents the repository assigned to the crate
    pub repository: String,
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
    pub fn load(manifest_path: Option<&str>) -> Result<ArchConfig> {
        let mut content = String::new();
        let path = format!(
            "{}/Cargo.toml",
            match manifest_path {
                Some(val) => val.to_string(),
                None => match std::env::var("CARGO_MANIFEST_DIR") {
                    Ok(val) => val,
                    Err(_) => ".".to_string(),
                },
            }
        );
        let mut path = File::open(path.as_str()).context("Unable to open Cargo.toml")?;
        path.read_to_string(&mut content)
            .context("cargo-arch: invalid or missing Cargo.toml options")?;
        Ok(toml::from_str::<Cargo>(&content)
            .context("cargo-arch: could not decode manifest")?
            .into())
    }

    pub fn generate_pkgbuild(&self, src: core::Source) -> Result<()> {
        let mut file = File::create("PKGBUILD").context("unable to create PKGBUILD")?;

        macro_rules! add_data {
            ( $fmt: expr, $data: expr ) => {
                writeln!(file, "{}", format!($fmt, $data)).context("failed to write to PKGBUILD")?
            };
        }

        fn quote_data(data: &Vec<String>) -> String {
            format!("\"{}\"", data.join("\" \""))
        }

        for i in &self.maintainers {
            add_data!("# Maintainer: {}", i);
        }

        let mut makedepends = self.makedepends.clone();

        // git-makedepends is required on git-mode
        if src == core::Source::Git && !makedepends.contains(&"git".to_string()) {
            makedepends.push("git".to_string());
        }

        add_data!("pkgname={}", self.pkgname);
        add_data!("pkgver={}", self.pkgver.replace("-", "_"));
        add_data!("pkgrel={}", self.pkgrel);
        if self.epoch != 0 {
            add_data!("epoch={}", self.epoch);
        }
        add_data!("pkgdesc=\"{}\"", self.pkgdesc);
        if !self.url.is_empty() {
            add_data!("url=\"{}\"", self.url);
        } else if src == core::Source::Git {
            // Use repository as url for the new PKGBUILD
            add_data!("url=\"{}\"", self.repository);
        }

        if !self.license.is_empty() {
            add_data!("license=({})", quote_data(&self.license));
        }
        if !self.install.is_empty() {
            add_data!("install=\"{}\"", self.install);
        }
        if !self.changelog.is_empty() {
            add_data!("changelog=\"{}\"", self.changelog);
        }
        if !self.source.is_empty() {
            add_data!("source=({})", quote_data(&self.source));
        } else if src == core::Source::Git {
            // If no custom source was specified and buildmode is git, use url+git
            add_data!(
                "source=({})",
                quote_data(&vec!["git+$url#tag=v$pkgver".to_owned()])
            );
        }

        if !self.validpgpkeys.is_empty() {
            add_data!("validpgpkeys=({})", quote_data(&self.validpgpkeys));
        }
        if !self.noextract.is_empty() {
            add_data!("noextract=({})", quote_data(&self.noextract));
        }
        if !self.md5sums.is_empty() {
            add_data!("md5sums=({})", quote_data(&self.md5sums));
        } else if src == core::Source::Git {
            // On gitmode, skip md5sum verification
            add_data!("md5sums=({})", quote_data(&vec!["SKIP".to_owned()]));
        }

        if !self.sha1sums.is_empty() {
            add_data!("sha1sums=({})", quote_data(&self.sha1sums));
        }
        if !self.sha256sums.is_empty() {
            add_data!("sha256sums=({})", quote_data(&self.sha256sums));
        }
        if !self.sha384sums.is_empty() {
            add_data!("sha384sums=({})", quote_data(&self.sha384sums));
        }
        if !self.sha512sums.is_empty() {
            add_data!("sha512sums=({})", quote_data(&self.sha512sums));
        }
        if !self.groups.is_empty() {
            add_data!("groups=({})", quote_data(&self.groups));
        }
        add_data!("arch=({})", quote_data(&self.arch));
        if !self.backup.is_empty() {
            add_data!("backup=({})", quote_data(&self.backup));
        }
        if !self.depends.is_empty() {
            add_data!("depends=({})", quote_data(&self.depends));
        }
        if !makedepends.is_empty() {
            add_data!("makedepends=({})", quote_data(&makedepends));
        }
        if !self.checkdepends.is_empty() {
            add_data!("checkdepends=({})", quote_data(&self.checkdepends));
        }
        if !self.optdepends.is_empty() {
            add_data!("optdepends=({})", quote_data(&self.optdepends));
        }
        if !self.conflicts.is_empty() {
            add_data!("conflicts=({})", quote_data(&self.conflicts));
        }
        if !self.provides.is_empty() {
            add_data!("provides=({})", quote_data(&self.provides));
        }
        if !self.replaces.is_empty() {
            add_data!("replaces=({})", quote_data(&self.replaces));
        }
        if !self.options.is_empty() {
            add_data!("options=({})", quote_data(&self.options));
        }

        let to_write_pkgbuild = match src {
            super::Source::Crates => include_str!("PKGBUILD-TEMPLATE_crates"),
            super::Source::Git => include_str!("PKGBUILD-TEMPLATE_git"),
        };

        writeln!(file, "\n{}", to_write_pkgbuild).context("failed to write to PKGBUILD")
    }
}

impl From<Cargo> for ArchConfig {
    fn from(cargo: Cargo) -> Self {
        let arch_config = cargo.package.metadata.arch;

        let maintainers = arch_config
            .maintainers
            .as_ref()
            .unwrap_or(&cargo.package.authors)
            .clone();
        let repository = cargo
            .package
            .repository
            .clone()
            .unwrap_or("".to_owned())
            .clone();
        let pkgname = arch_config
            .pkgname
            .as_ref()
            .unwrap_or(&cargo.package.name)
            .clone();
        let pkgver = arch_config
            .pkgver
            .as_ref()
            .unwrap_or(&cargo.package.version)
            .clone();
        let pkgrel = arch_config
            .pkgrel
            .as_ref()
            .unwrap_or(&"1".to_string())
            .clone();
        let pkgdesc = arch_config
            .pkgdesc
            .as_ref()
            .unwrap_or(&cargo.package.description)
            .clone();
        let url = arch_config
            .url
            .as_ref()
            .or(cargo.package.homepage.as_ref())
            .or(cargo.package.repository.as_ref())
            .unwrap_or(&String::new())
            .clone();
        let license = arch_config
            .license
            .as_ref()
            .unwrap_or(
                &cargo
                    .package
                    .license
                    .split("/")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
            .clone();

        ArchConfig {
            maintainers,
            pkgname,
            pkgver,
            pkgrel,
            epoch: arch_config.epoch,
            pkgdesc,
            url,
            repository,
            license,
            install: arch_config.install,
            changelog: arch_config.changelog,
            source: arch_config.source,
            validpgpkeys: arch_config.validpgpkeys,
            noextract: arch_config.noextract,
            md5sums: arch_config.md5sums,
            sha1sums: arch_config.sha1sums,
            sha256sums: arch_config.sha256sums,
            sha384sums: arch_config.sha384sums,
            sha512sums: arch_config.sha512sums,
            groups: arch_config.groups,
            arch: arch_config.arch,
            backup: arch_config.backup,
            depends: arch_config.depends,
            makedepends: arch_config.makedepends,
            checkdepends: arch_config.checkdepends,
            optdepends: arch_config.optdepends,
            conflicts: arch_config.conflicts,
            provides: arch_config.provides,
            replaces: arch_config.replaces,
            options: arch_config.options,
        }
    }
}
