use clap::{ArgGroup, Args};

use super::Run;
use crate::{db::Config, iter_exclude, rclone};

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
#[command(group(
    ArgGroup::new("selection")
        .required(true)
        // only one of them allowed
        .multiple(false)
        .args(["all","source"])
))]
pub struct Copy {
    #[arg(name = "source", short, long)]
    src: Option<String>,
    #[arg(short, long)]
    exclude: Option<Vec<String>>,
    #[arg(short, long)]
    all: bool,
}

impl Run for Copy {
    fn run(&mut self) {
        let db = Config::load();
        let entries = if self.all {
            db.mappings.as_vec()
        } else {
            let entry = db
                .mappings
                .get(self.src.as_ref().unwrap())
                .expect("Only Declared Mappings Can Be Copied");
            vec![entry]
        };
        let exclude: Vec<&str> = db
            .get_excluded()
            .chain(iter_exclude!(self.exclude))
            .collect();
        for entry in entries {
            let exclude = entry.excluded()
            .chain(exclude.clone());
            println!(
                "{:?}",
                rclone::copy(
                    entry.local_path.as_str(),
                    entry.remote_path.as_str(),
                    Some(exclude)
                )
            );
        }
    }
    // .chain(iter_exclude!(entry; |e| iter_exclude!(e.exclude => std::convert::identity )));
}
