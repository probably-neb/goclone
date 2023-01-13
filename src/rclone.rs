use anyhow::{anyhow, Context, Result};
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
        false => Err(anyhow!(std::str::from_utf8(&output.stderr)?.to_string())),
    }
}

fn rclone() -> Command {
    return Command::new("rclone");
}

pub fn exists(path: &str) -> Result<String> {
    run(rclone().arg("lsf").arg(path).arg("--dry-run"))
}

pub fn copy(src: &str, dest: &str, exclude: Option<impl Iterator<Item= &str>>) -> Result<String> {
    let mut cmd = rclone();
    cmd.arg("copy").arg(src).arg(dest);
    exclude_all(&mut cmd, exclude);
    run(&mut cmd)
}

pub fn exclude(cmd: &mut Command, to_exclude: &str) {
    cmd.args(["--exclude", to_exclude]);
}

pub fn exclude_all(cmd: &mut Command, excluded: Option<impl Iterator<Item = &str>>) {
    if let Some(excluded) = excluded {
        for exc in excluded {
            exclude(cmd,exc)
        }
    }
}

pub(crate) fn ls(path: &str, excluded: Option<impl Iterator<Item = &str>>) -> Result<String> {
    let mut cmd = rclone();
    cmd.arg("ls").arg(path);
    exclude_all(&mut cmd, excluded);
    run(&mut cmd)
}
