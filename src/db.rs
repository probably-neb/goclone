use std::fs::File;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

const DB_PATH: &str = "./db.json";

#[derive(Debug)]
pub struct DB {
    pub map: BiMap,
    path_str: String,
}

pub struct Entry {
    pub remote_path: String,
    pub local_path: String,
}

#[derive(Deserialize,Serialize,Debug, Default)]
pub struct BiMap {
    pub remote_to_local: HashMap<String,String>,
    pub local_to_remote: HashMap<String,String>,
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
    pub fn load() -> Self {
        let mut db = Self::default();
        if let Ok(mut file) = File::open(&db.path_str) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("DB Readable");
            db.map = serde_json::from_str(&contents).expect("DB deserializable");
        }
        return db;
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        // TODO: load/store implementations
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
