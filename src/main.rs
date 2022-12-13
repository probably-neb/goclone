use clap::{Parser,Args, Subcommand};
use enum_dispatch::enum_dispatch;
mod db;
use db::{DB,Entry};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Be Verbose
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    #[command(subcommand)]
    subcommand: SubCommands
}

#[enum_dispatch]
#[derive(Subcommand, Debug)]
enum SubCommands {
    // /// Copy the files from a given path to the mapped destination
    // #[command(author,version,about,long_about=None)]
    // Copy(Copy),
    /// Add a mapping between a remote and local path
    #[command(author,version,about,long_about=None)]
    Add(Add)
}

#[enum_dispatch(SubCommands)]
trait Run {
    fn pre_run(&self) {}
    fn run(&self);
    fn post_run(&self) {}
    fn run_all(&self) {
        self.pre_run();
        self.run();
        self.post_run();
    }
}

#[derive(Args,Debug)]
struct Copy;

// TODO: add checks and --no-check flag
#[derive(Args, Debug, Clone)]
struct Add {
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

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);
    cli.subcommand.run_all();
    Ok(())
}
