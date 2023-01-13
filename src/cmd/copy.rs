use clap::Args;

use super::Run;
use crate::{
    db::Config,
    rclone, iter_exclude,
};

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
pub struct Copy {
    #[arg(name = "source", short, long)]
    src: String,
    #[arg(short, long)]
    exclude: Option<Vec<String>>,
}

impl Run for Copy {
    fn run(&mut self) {
        let db = Config::load();
        let entry = db.mappings.get(&self.src);
        let exclude = db.get_excluded()
            .chain(iter_exclude!(entry; |e| iter_exclude!(e.exclude => std::convert::identity)))
            .chain(iter_exclude!(self.exclude));
        let entry = entry.expect("Only Declared Mappings Can Be Copied");
            // .chain(iter_exclude!(entry; |e| iter_exclude!(e.exclude => std::convert::identity )));
        println!("{:?}",rclone::copy(entry.local_path.as_str(), entry.remote_path.as_str(), Some(exclude)));
    }
}
