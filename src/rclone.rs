use std::process::{Command, Output};
use anyhow::{Result, Context};

pub struct Rclone { }

impl Rclone {
    pub fn shell() -> Command {
        return Command::new("rclone");
    }

    pub fn exists(path: &String) -> Result<Output> {
        Self::shell()
            .arg("ls")
            .arg(path)
            .arg("--dry-run")
            .output()
            .with_context(|| "failed")
    }
}
