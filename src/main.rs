#![feature(anonymous_lifetime_in_impl_trait)]

mod db;
mod cmd;
mod rclone;
mod util;
use cmd::{Cli, Parser, Run};

fn main() {
    let mut cli = Cli::parse();
    cli.run();
}
