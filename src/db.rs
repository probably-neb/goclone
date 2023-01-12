use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const DB_PATH: &str = "./goclone.toml";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub map: PathMap,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct PathMap {
    pub local_to_remote: HashMap<String, String>,
}

impl PathMap {
    pub fn as_vec(&self) -> Vec<Entry> {
        let entries: Vec<Entry> = self
            .local_to_remote
            .iter()
            .map(|(local_path, remote_path)| Entry {
                local_path: local_path.clone(),
                remote_path: remote_path.clone(),
            })
            .collect();
        return entries;
    }

    pub fn insert(&mut self, entry: Entry) {
        self.local_to_remote
            .insert(entry.local_path, entry.remote_path);
    }
}

impl Config {
    // TODO: Make this return a Result<Self>
    pub fn load() -> Self {
        let mut file = File::open(DB_PATH).expect("config openable");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("DB Readable");

        let config = match toml::from_str(contents.as_str()) {
            Ok(config) => config,
            Err(_) => Default::default(),
        };
        return config;
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        // TODO: load/store implementations
        File::create(DB_PATH)
            .expect("db can be opened")
            .write_all(
                toml::to_string(&self)
                    .expect("DB is serializable")
                    .as_bytes(),
            )
            .expect("DB is writeable");
    }
}

/// Used to avoid positional args in BiMap
pub struct Entry {
    pub remote_path: String,
    pub local_path: String,
}
