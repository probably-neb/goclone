use enum_dispatch::enum_dispatch;
use clap::Subcommand;
pub use clap::Parser;

mod add;
use add::Add;
mod list;
use list::List;
mod check;
use check::CheckDb;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Be Verbose
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    #[command(subcommand)]
    pub subcommand: SubCommands
}

#[enum_dispatch]
#[derive(Subcommand, Debug)]
pub enum SubCommands {
    // /// Copy the files from a given path to the mapped destination
    // #[command(author,version,about,long_about=None)]
    // Copy(Copy),
    /// Add a mapping between a remote and local path
    #[command(author,version,about,long_about=None)]
    Add(Add),
    List(List),
    CheckDb(CheckDb),
}

#[enum_dispatch(SubCommands)]
pub trait Run {
    fn pre_run(&self) {}
    fn run(&self);
    fn post_run(&self) {}
    fn run_all(&self) {
        self.pre_run();
        self.run();
        self.post_run();
    }
}

// #[derive(Args,Debug)]
// struct Copy;

