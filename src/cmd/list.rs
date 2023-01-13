use clap::Args;

use super::Run;
use crate::{db::Config, iter_exclude, rclone};

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
pub struct List {
    #[arg(short, long)]
    path: Option<String>,
    #[arg(short, long)]
    exclude: Option<Vec<String>>,
}

impl Run for List {
    fn run(&mut self) {
        let db = Config::load();
        match &self.path {
            Some(path) => {
                let entry = db
                    .mappings
                    .get(path.as_str())
                    .expect("Only Configured Mappings Can Be Listed");
                let excluded = db
                    .get_excluded()
                    .chain(iter_exclude!(self.exclude))
                    .chain(iter_exclude!(entry.exclude));

                println!("{}", rclone::ls(path.as_str(), Some(excluded)).unwrap());
            }
            None => {
                println!("<local> -> <remote>");
                for entry in db.mappings.as_vec() {
                    println!("{} -> {}", entry.local_path, entry.remote_path);
                }
            }
        }
    }
}
