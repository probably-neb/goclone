pub use clap::Parser;
use clap::Subcommand;
use enum_dispatch::enum_dispatch;

// Commands
mod add;
mod check;
mod copy;
mod list;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Be Verbose
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    #[command(subcommand)]
    pub subcommand: SubCommands,
}

impl Run for Cli {
    fn run(&mut self) {
        self.subcommand.run_all();
    }
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
    fn pre_run(&mut self) {}
    fn run(&mut self);
    fn post_run(&mut self) {}
    fn run_all(&mut self) {
        self.pre_run();
        self.run();
        self.post_run();
    }
}

// #[derive(Args,Debug)]
// struct Copy;
