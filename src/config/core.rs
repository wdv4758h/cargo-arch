use super::meta::CargoMetadata;


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
    pub authors: Vec<String>,
    pub license: String,    // Multiple licenses are separated by `/`
    pub readme: String,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub metadata: Option<CargoMetadata>,
}


pub trait ToPackageConfig<T> {
    fn to_config(&self) -> T;
}

pub trait GeneratePackage {
    fn generate_package_config(&self);
}
