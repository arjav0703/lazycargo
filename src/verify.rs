pub fn verfiy_rust_project() -> Result<(), &'static str> {
    match std::path::Path::new("Cargo.toml").exists() {
        true => Ok(()),
        false => {
            Err("Cargo.toml not found. Please run this command in the root of a Rust project.")
        }
    }
}
