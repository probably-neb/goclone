use clap::Args;

use super::Run;
use crate::db::{DB,Entry};

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
pub struct Add {
    /// the remote path
    #[arg(name="remote", short, long)]
    remote_path: String,
    /// the local path
    #[arg(name="local", short, long)]
    local_path: String,
}

impl Add {
    fn as_entry(&self) -> Entry {
        return self.clone().into();
    }
}

impl Run for Add {
    fn run(&self) {
        let mut db = DB::load();
        db.insert(self.as_entry());
        println!("{db:?}");
    }
}

impl From<Add> for Entry {
    fn from(a: Add) -> Self {
        let local_path = a.local_path;
        let remote_path = a.remote_path;
        return Self {remote_path, local_path};
    }
}
