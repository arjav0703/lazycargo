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

pub trait DependencyExtractor {
    fn get_dependencies(&self) -> Vec<Dependency>;
}

impl DependencyExtractor for Manifest {
    fn get_dependencies(&self) -> Vec<Dependency> {
        let mut dependencies = Vec::new();

        if let Some(deps) = &self.dependencies {
            for (name, dep) in deps {
                let (version, features) = match dep {
                    cargo_manifest::Dependency::Simple(version) => {
                        (version.clone(), Feature::simple())
                    }
                    cargo_manifest::Dependency::Detailed(details) => {
                        let version = details.version.clone().unwrap_or_default();
                        let features = Feature::from_detail(
                            details.features.as_ref(),
                            details.default_features,
                        );
                        (version, features)
                    }
                    cargo_manifest::Dependency::Inherited(_) => (String::new(), Feature::simple()),
                };
                dependencies.push(Dependency {
                    name: name.clone(),
                    version,
                    status: DependencyType::Regular,
                    features,
                });
            }
        }

        if let Some(dev_deps) = &self.dev_dependencies {
            for (name, dep) in dev_deps {
                let (version, features) = match dep {
                    cargo_manifest::Dependency::Simple(version) => {
                        (version.clone(), Feature::simple())
                    }
                    cargo_manifest::Dependency::Detailed(details) => {
                        let version = details.version.clone().unwrap_or_default();
                        let features = Feature::from_detail(
                            details.features.as_ref(),
                            details.default_features,
                        );
                        (version, features)
                    }
                    cargo_manifest::Dependency::Inherited(_) => (String::new(), Feature::simple()),
                };
                dependencies.push(Dependency {
                    name: name.clone(),
                    version,
                    status: DependencyType::Dev,
                    features,
                });
            }
        }

        dependencies
    }
}
