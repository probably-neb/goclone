use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, self};
use std::io::prelude::*;
use toml_edit::easy as toml;

const DB_PATH: &str = "./goclone.toml";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub exclude: Option<Vec<String>>,
    pub mappings: PathMap,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct PathMap(pub HashMap<String, String>);

impl PathMap {
    pub fn as_vec(&self) -> Vec<Entry> {
        let entries: Vec<Entry> = self
            .0
            .iter()
            .map(|(local_path, remote_path)| Entry {
                local_path: local_path.clone(),
                remote_path: remote_path.clone(),
            })
            .collect();
        return entries;
    }

    pub fn get(&self, from: &str) -> Option<Entry> {
        // Naively assumes from is local path
        // FIXME: check for remote paths as well
        // and come up with way to resolve local path from remote
        let from = Self::canonicalize_path(from);
        let remote_path = self.0.get(&from)?.clone();
        return Some(Entry {
            remote_path,
            local_path: from,
        });
    }

    fn canonicalize_path(local_path: &str) -> String {
        fs::canonicalize(local_path).expect("File Exists").into_os_string().into_string().unwrap() 
    }

    pub fn insert(&mut self, entry: Entry) {
        self.0
            .insert(dbg!(Self::canonicalize_path(entry.local_path.as_str())), entry.remote_path);
    }

}

impl Config {
    // TODO: Make this return a Result<Self>
    pub fn load() -> Self {
        let contents = Self::load_config_file();
        Self::_load(contents.as_str())
    }

    fn load_config_file() -> String {
        let mut file = File::open(DB_PATH).expect("config openable");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("DB Readable");

        return contents;
    }

    fn _load(contents: &str) -> Self {
        let config = match toml::from_str(contents) {
            Ok(config) => config,
            // FIXME: return error on config error, default on empty
            Err(_) => Default::default(),
        };
        return config;
    }

    pub fn _write(contents: &str) {
        File::create(DB_PATH)
            .expect("db can be opened")
            .write_all(
                contents
                    .as_bytes(),
            )
            .expect("DB is writeable");
    }

    // pub fn write(&self) {
    //     Self::_write(
    //             toml::to_string_pretty(&self)
    //                 .expect("DB is serializable")
    //                 .as_str()
    //     )
    // }

    pub fn add_mapping(&mut self, entry: Entry) {
        // load and modify config file contents using toml_edit
        let contents = Self::load_config_file();
        let mut doc = contents.parse::<toml_edit::Document>().expect("Invalid Config");
        
        doc["mappings"][entry.local_path.as_str()] = toml_edit::value(entry.remote_path.as_str());

        Self::_write( doc.to_string().as_str());

        // update self with this change as well
        // 
        // doubles as an excuse to make self mut like it should be 
        // without clippy yelling at me for making it mut when it 
        // doesn't have to be
        self.mappings.insert(entry);
    }
}

/// Used to avoid positional args
pub struct Entry {
    pub remote_path: String,
    pub local_path: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_direct_mapping() {
        let toml = r#"
[mappings]
"/dev/null" = "dropbox:"
"#;
        let config = Config::_load(toml);
        let mappings: HashMap<String, String> = [("/dev/null".to_string(),"dropbox:".to_string())].into_iter().collect();
        assert!(config.mappings.0 == mappings,"{:?}", config.mappings);
    }
}
