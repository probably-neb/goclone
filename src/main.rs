mod db;
mod cmd;
mod rclone;
use cmd::{Cli, Parser, Run};

fn main() {
    let mut cli = Cli::parse();
    cli.run();
}
