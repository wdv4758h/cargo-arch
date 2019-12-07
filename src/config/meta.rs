//! Metadata for different platform's package

use super::arch::CargoArch;
use std::default::Default;

/// data in `[package.metadata]` section
#[derive(Clone, Debug, Deserialize)]
pub struct CargoMetadata {
    #[serde(default)]
    pub arch: CargoArch,
}

impl Default for CargoMetadata {
    // if we do not decode an empty [arch] block then the defaults are
    // not actually populated
    fn default() -> Self {
        toml::from_str("[arch]").unwrap()
    }
}
