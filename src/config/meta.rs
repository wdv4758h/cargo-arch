//! Metadata for different platform's package

use serde::Deserialize;
use super::arch::CargoArch;
use std::default::Default;

/// data in `[package.metadata]` section
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CargoMetadata {
    #[serde(default)]
    pub arch: CargoArch,
}
