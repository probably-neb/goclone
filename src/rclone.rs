use anyhow::{Context, Result};
use std::process::{Command, Output};

fn rclone() -> Command {
    return Command::new("rclone");
}

pub fn exists(path: &String) -> Result<Output> {
    rclone()
        .arg("ls")
        .arg(path)
        .arg("--dry-run")
        .output()
        .with_context(|| "failed")
}
