use clap::Args;

use super::Run;
use crate::db::Config;

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
pub struct List {
    // #[arg(name="remote", short, long)]
    // remote_path: String,
    // #[arg(name="local", short, long)]
    // local_path: String,
}

impl Run for List {
    fn run(&self) {
        let db = Config::load();
        println!("<local> -> <remote>");
        for entry in db.mappings.as_vec() {
            println!("{} -> {}", entry.local_path, entry.remote_path);
        }
    }
}
