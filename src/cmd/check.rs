use clap::Args;

use super::Run;
use crate::{rclone::Rclone, db::Config};

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
pub struct CheckDb {
    // #[arg(name="remote", short, long)]
    // remote_path: String,
    // #[arg(name="local", short, long)]
    // local_path: String,
}

impl Run for CheckDb {
    fn run(&self) {
        let db = Config::load();
        println!("<local> -> <remote>");
        for entry in db.map.as_vec() {
            let both_exist = Rclone::exists(&entry.remote_path).and(Rclone::exists(&entry.local_path));
            let status = match both_exist {
                Ok(_) => "Ok".to_string(),
                Err(err) => format!("{:?}", err)
            };
            println!("{} -> {} ... {}", entry.local_path, entry.remote_path, status);
        }
    }
}
