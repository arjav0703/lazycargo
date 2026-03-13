#[derive(Debug, Clone, PartialEq)]
pub enum CargoCommand {
    Build,
    Check,
    Test,
    Clippy,
    Fmt,
    Clean,
}

impl CargoCommand {
    pub fn all() -> Vec<CargoCommand> {
        vec![
            CargoCommand::Build,
            CargoCommand::Check,
            CargoCommand::Test,
            CargoCommand::Clippy,
            CargoCommand::Fmt,
            CargoCommand::Clean,
        ]
    }

    pub fn label(&self) -> &str {
        match self {
            CargoCommand::Build => "build",
            CargoCommand::Check => "check",
            CargoCommand::Test => "test",
            CargoCommand::Clippy => "clippy",
            CargoCommand::Fmt => "fmt",
            CargoCommand::Clean => "clean",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            CargoCommand::Build => "Compile the current package",
            CargoCommand::Check => "Check for errors without producing a binary",
            CargoCommand::Test => "Run all tests",
            CargoCommand::Clippy => "Run lints via Clippy",
            CargoCommand::Fmt => "Format source code",
            CargoCommand::Clean => "Remove generated artifacts",
        }
    }

    pub fn args(&self) -> Vec<&str> {
        match self {
            CargoCommand::Build => vec!["build"],
            CargoCommand::Check => vec!["check"],
            CargoCommand::Test => vec!["test"],
            CargoCommand::Clippy => vec!["clippy", "--", "-D", "warnings"],
            CargoCommand::Fmt => vec!["fmt"],
            CargoCommand::Clean => vec!["clean"],
        }
    }
}
