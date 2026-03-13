use super::*;

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
