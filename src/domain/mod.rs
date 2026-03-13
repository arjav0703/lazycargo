pub mod dependencies;
pub use dependencies::*;

use cargo_manifest::Manifest;
pub fn get_manifest() -> Manifest {
    Manifest::from_path("Cargo.toml").expect("Failed to read Cargo.toml")
}
