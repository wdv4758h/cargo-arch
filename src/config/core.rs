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
    pub license: String,    // Multiple licenses are separated by `/`
    pub readme: String,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub metadata: Option<CargoMetadata>,
}


/// A trait for making specific platform package config's settings
pub trait ToPackageConfig<T> {
    fn to_config(&self) -> T;
}

/// A trait for generate specific platform package's config
pub trait GeneratePackageConfig {
    fn generate_package_config(&self);
}
