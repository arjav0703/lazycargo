use super::*;

impl App {
    pub fn run_cargo_add(&mut self, crate_name: String) {
        let cmd = format!("cargo add {}", crate_name);
        self.last_command = Some(cmd);
        self.command_status = CommandStatus::Running;
        self.output_scroll = 0;

        let tx = self.cmd_result_tx.clone();
        tokio::spawn(async move {
            let output = tokio::process::Command::new("cargo")
                .args(["add", &crate_name])
                .env("CARGO_TERM_COLOR", "always")
                .output()
                .await
                .expect("Failed to run cargo add");

            let combined = [output.stdout, output.stderr].concat();
            let lines: Vec<String> = String::from_utf8_lossy(&combined)
                .lines()
                .map(|l| l.to_string())
                .collect();

            let _ = tx
                .send(CommandResult {
                    lines,
                    success: output.status.success(),
                    reload_manifest: output.status.success(),
                })
                .await;
        });
    }

    pub fn run_cargo_command(&mut self, command: CargoCommand) {
        let args: Vec<String> = command.args().into_iter().map(|s| s.to_string()).collect();
        let cmd_str = format!("cargo {}", args.join(" "));
        self.last_command = Some(cmd_str);
        self.command_status = CommandStatus::Running;
        self.output_scroll = 0;

        let tx = self.cmd_result_tx.clone();
        tokio::spawn(async move {
            let output = tokio::process::Command::new("cargo")
                .args(&args)
                .env("CARGO_TERM_COLOR", "always")
                .output()
                .await
                .expect("Failed to run cargo command");

            let combined = [output.stdout, output.stderr].concat();
            let lines: Vec<String> = String::from_utf8_lossy(&combined)
                .lines()
                .map(|l| l.to_string())
                .collect();

            let _ = tx
                .send(CommandResult {
                    lines,
                    success: output.status.success(),
                    reload_manifest: false,
                })
                .await;
        });
    }

    pub fn run_cargo_remove(&mut self, crate_name: String) {
        let cmd = format!("cargo remove {}", crate_name);
        self.last_command = Some(cmd);
        self.command_status = CommandStatus::Running;
        self.output_scroll = 0;

        let tx = self.cmd_result_tx.clone();
        tokio::spawn(async move {
            let output = tokio::process::Command::new("cargo")
                .args(["remove", &crate_name])
                .env("CARGO_TERM_COLOR", "always")
                .output()
                .await
                .expect("Failed to run cargo remove");

            let combined = [output.stdout, output.stderr].concat();
            let lines: Vec<String> = String::from_utf8_lossy(&combined)
                .lines()
                .map(|l| l.to_string())
                .collect();

            let _ = tx
                .send(CommandResult {
                    lines,
                    success: output.status.success(),
                    reload_manifest: output.status.success(),
                })
                .await;
        });
    }
}
