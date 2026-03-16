use super::*;

impl App {
    pub async fn run_cargo_add(&mut self, crate_name: String) {
        let cmd = format!("cargo add {}", crate_name);
        self.last_command = Some(cmd.clone());
        self.command_status = CommandStatus::Running;
        self.output_scroll = 0;

        let exit_code: i32 = std::process::Command::new("cargo")
            .args(["add", &crate_name])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to run cargo add")
            .wait()
            .expect("Failed to wait on cargo add")
            .code()
            .unwrap_or(1);

        self.command_status = if exit_code == 0 {
            if let Ok(manifest) = Manifest::from_path("Cargo.toml") {
                self.manifest = manifest;
            }
            self.dependencies = self.manifest.get_dependencies();
            self.dep_list_state.select(Some(0));
            CommandStatus::Success
        } else {
            CommandStatus::Failed
        };
    }

    pub async fn run_cargo_command(&mut self, command: CargoCommand) {
        let args = command.args();
        let cmd_str = format!("cargo {}", args.join(" "));
        self.last_command = Some(cmd_str);
        self.command_status = CommandStatus::Running;
        self.output_scroll = 0;

        let output = std::process::Command::new("cargo")
            .args(&args)
            .env("CARGO_TERM_COLOR", "always")
            .output()
            .expect("Failed to run cargo command");

        let combined = [output.stdout, output.stderr].concat();
        let lines: Vec<String> = String::from_utf8_lossy(&combined)
            .lines()
            .map(|l| l.to_string())
            .collect();

        *self.main_output_lines.lock().unwrap() = lines;

        self.command_status = if output.status.success() {
            CommandStatus::Success
        } else {
            CommandStatus::Failed
        };
    }

    pub async fn run_cargo_remove(&mut self, crate_name: String) {
        let cmd = format!("cargo remove {}", crate_name);
        self.last_command = Some(cmd.clone());
        self.command_status = CommandStatus::Running;
        self.output_scroll = 0;

        let exit_code: i32 = std::process::Command::new("cargo")
            .args(["remove", &crate_name])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to run cargo remove")
            .wait()
            .expect("Failed to wait on cargo remove")
            .code()
            .unwrap_or(1);

        self.command_status = if exit_code == 0 {
            if let Ok(manifest) = Manifest::from_path("Cargo.toml") {
                self.manifest = manifest;
            }
            self.dependencies = self.manifest.get_dependencies();
            if self.dependencies.is_empty() {
                self.dep_list_state.select(None);
            } else {
                self.dep_list_state.select(Some(0));
            }
            CommandStatus::Success
        } else {
            CommandStatus::Failed
        };
    }
}
