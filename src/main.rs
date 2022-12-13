mod db;
mod cmd;
use cmd::{Cli, Parser, Run};

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    cli.subcommand.run_all();
}
