//! Metadata for different platform's package

use super::arch::CargoArch;


/// data in `[package.metadata]` section
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CargoMetadata {
    pub arch: Option<CargoArch>,
}
