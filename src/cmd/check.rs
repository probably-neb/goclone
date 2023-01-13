use clap::Args;

use super::Run;
use crate::{db::Config, rclone};

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
pub struct CheckDb {}

impl Run for CheckDb {
    fn run(&self) {
        let db = Config::load();
        println!("<local> -> <remote>");
        for entry in db.mappings.as_vec() {
            let both_exist =
                rclone::exists(&entry.remote_path).and(rclone::exists(&entry.local_path));
            let status = match both_exist {
                Ok(_) => "Ok".to_string(),
                Err(err) => format!("{:?}", err),
            };
            println!(
                "{} -> {} ... {}",
                entry.local_path, entry.remote_path, status
            );
        }
    }
}
