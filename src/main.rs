mod db;
mod cmd;
mod rclone;
use cmd::{Cli, Parser, Run};

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    cli.subcommand.run_all();
}
