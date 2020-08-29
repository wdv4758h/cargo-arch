//! Basic Rust package's config, modified from Cargo.

use super::meta::CargoMetadata;

/// data in Cargo.toml
#[derive(Clone, Debug, Deserialize)]
pub struct Cargo {
    pub package: CargoPackage,
}

/// data in `[package]` section
#[derive(Clone, Debug, Deserialize)]
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: String, // Multiple licenses are separated by `/`
    pub readme: String,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    #[serde(default)]
    pub metadata: CargoMetadata,
}

#[derive(Debug, PartialEq)]
pub enum Source {
    Crates,
    Git,
}

impl std::str::FromStr for Source {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "git" => Source::Git,
            "crates.io" => Source::Crates,
            _ => panic!("invalid source"),
        })
    }
}
