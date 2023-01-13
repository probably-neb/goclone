use clap::Args;

use super::Run;
use crate::{
    db::Config,
    rclone,
};

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
pub struct Copy {
    #[arg(name = "source", short, long)]
    src: String,
}

impl Run for Copy {
    fn run(&mut self) {
        let db = Config::load();
        let entry = db.mappings.get(&self.src).expect("Can only copy item with declared mapping");
        println!("{:?}",rclone::copy(entry.local_path.as_str(), entry.remote_path.as_str()));
    }
}
