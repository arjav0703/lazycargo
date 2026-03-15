use super::*;

impl App {
    pub async fn run_cargo_add(&mut self, crate_name: String) {
        let cmd = format!("cargo add {}", crate_name);
        self.last_command = Some(cmd.clone());
        self.command_status = CommandStatus::Running;

        let exit_code: i32 = std::process::Command::new("cargo")
            .args(["add", &crate_name])
            .spawn()
            .expect("Failed to run cargo add")
            .wait()
            .expect("Failed to wait on cargo add")
            .code()
            .unwrap_or(1);

        self.command_status = if exit_code == 0 {
            self.dependencies = self.manifest.get_dependencies();
            self.dep_list_state.select(Some(0));
            CommandStatus::Success
        } else {
            CommandStatus::Failed
        };
    }

    pub async fn run_cargo_remove(&mut self, crate_name: String) {
        let cmd = format!("cargo remove {}", crate_name);
        self.last_command = Some(cmd.clone());
        self.command_status = CommandStatus::Running;

        let exit_code: i32 = std::process::Command::new("cargo")
            .args(["remove", &crate_name])
            .spawn()
            .expect("Failed to run cargo remove")
            .wait()
            .expect("Failed to wait on cargo remove")
            .code()
            .unwrap_or(1);

        self.command_status = if exit_code == 0 {
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
