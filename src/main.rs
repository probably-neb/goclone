use clap::{Parser,Args, Subcommand, ValueEnum};
use serde::{Serialize,Deserialize};
use std::{collections::hash_map::HashMap, path::Path,fs::File, error::Error, ops::{Deref, DerefMut}, io::Write};
use enum_dispatch::enum_dispatch;

const DB_PATH: &str = "./db";

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
        let mut db = DB::default();
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

struct Entry {
    remote_path: String,
    local_path: String,
}

#[derive(Deserialize,Serialize,Debug, Default)]
struct BiMap {
    remote_to_local: HashMap<String,String>,
    local_to_remote: HashMap<String,String>,
}

#[derive(Debug)]
struct DB {
    map: BiMap,
    path_str: String,
}

impl Default for DB {
    fn default() -> Self {
        Self { path_str: DB_PATH.to_string(), map: Default::default() }
    }
}

impl Deref for DB {
    type Target = BiMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for DB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl DB {
    fn path(&self) -> &Path {
        return Path::new(&self.path_str);
    }
}
impl Drop for DB {
    fn drop(&mut self) {
        File::create(&self.path_str)
            .expect("db can be opened")
            .write_all(serde_json::to_string(&self.map)
            .expect("DB is serializable").as_bytes())
            .expect("DB is writeable");
    }
}

impl BiMap {
    pub fn insert(&mut self, entry: Entry) {
        self.remote_to_local.insert(entry.remote_path.clone(), entry.local_path.clone());
        self.local_to_remote.insert(entry.local_path, entry.remote_path);
    }
}

fn main() -> std::io::Result<()> {
    let db_path = Path::new(DB_PATH);
    if !db_path.exists() {
        File::create(db_path)?;
    }
    let cli = Cli::parse();
    println!("{:?}", cli);
    cli.subcommand.run_all();
    Ok(())
}
