use super::arch::CargoArch;


/// data in [package.metadata]
#[derive(Clone, Debug, Default, RustcDecodable)]
pub struct CargoMetadata {
    pub arch: Option<CargoArch>,
}
