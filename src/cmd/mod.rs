use enum_dispatch::enum_dispatch;
use clap::Subcommand;
pub use clap::Parser;

// Commands
mod add;
mod list;
mod check;
mod copy;

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
    Add(add::Add),
    List(list::List),
    CheckDb(check::CheckDb),
    Copy(copy::Copy),
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

