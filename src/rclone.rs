use anyhow::{Context, Result, anyhow};
use std::process::Command;

// TODO: Create rclone error type and type alias for output String

fn run(cmd: &mut Command) -> Result<String> {
    let output = cmd.output().with_context(|| {
        format!(
            "cmd: `{:?} {:?}` failed to run",
            cmd.get_program(),
            cmd.get_args()
        )
    })?;

    match output.status.success() {
        true => Ok(std::str::from_utf8(&output.stdout)?.to_string()),
        false => Err(anyhow!(std::str::from_utf8(&output.stderr)?.to_string()))
    }
}

fn rclone() -> Command {
    return Command::new("rclone");
}

pub fn exists(path: &str) -> Result<String> {
    run(rclone().arg("lsf").arg(path).arg("--dry-run"))
}

pub fn copy(src: &str, dest: &str) -> Result<String> {
    run(rclone().arg("copy").arg(src).arg(dest))
}
