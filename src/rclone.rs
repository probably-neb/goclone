use std::process::{Command, Output};
use anyhow::{Result, anyhow, Context};

pub struct Rclone { }

impl Rclone {
    pub fn shell() -> Command {
        return Command::new("rclone");
    }
    pub fn copy(from: &String, to: &String) -> Result<Output> {
        Self::shell()
            .arg("copy")
            .arg(from)
            .arg(to)
            .output()
            .with_context(|| "Failed to Copy")
    }

    pub fn ls(path: &String) -> &mut Command {
        Self::shell()
            .arg("ls")
            .arg(path)
    }

    pub fn exists(path: &String) -> Result<Output> {
        Self::ls(path).arg("--dry-run")
            .output()
            .with_context(|| "failed")
    }
}
