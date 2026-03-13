mod extractor;
pub use extractor::*;

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub status: DependencyType,
    pub features: Feature,
}

#[derive(Debug)]
pub struct Feature {
    pub default_features: bool,
    pub extra_features: Vec<String>,
}

impl Feature {
    fn from_detail(extra: Option<&Vec<String>>, default_features: Option<bool>) -> Self {
        Feature {
            default_features: default_features.unwrap_or(true),
            extra_features: extra.cloned().unwrap_or_default(),
        }
    }

    fn simple() -> Self {
        Feature {
            default_features: true,
            extra_features: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum DependencyType {
    Regular,
    Dev,
}

use cargo_manifest::Manifest;

pub fn get_manifest() -> Manifest {
    Manifest::from_path("Cargo.toml").expect("Failed to read Cargo.toml")
}
