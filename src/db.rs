use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use toml_edit::easy as toml;

const DB_PATH: &str = "./goclone.toml";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub exclude: Option<Vec<String>>,
    pub mappings: PathMap,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct _Config {
    pub exclude: Option<Vec<String>>,
    pub mappings: _PathMap,
}

impl _Config {
    fn into_config(self) -> Result<Config> {
        let _Config { exclude, mappings } = self;
        let mappings = mappings.into_pathmap()?;
        Ok(Config { exclude, mappings })
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct PathMap(pub HashMap<String, Entry>);

#[derive(Deserialize, Serialize, Default, Debug)]
struct _PathMap(pub HashMap<String, toml::Value>);

impl _PathMap {
    fn into_pathmap(self) -> Result<PathMap> {
        self.0
            .into_iter()
            .map(|kv| match Entry::try_from(kv) {
                Ok(entry) => Ok((entry.local_path.clone(), entry)),
                Err(e) => Err(e),
            })
            .collect::<Result<_>>()
            .map(PathMap)
    }
}

impl PathMap {
    pub fn as_vec(&self) -> Vec<&Entry> {
        return self.0.values().collect();
    }

    // FIXME: Make this return &Entry
    pub fn get(&self, from: &str) -> Option<&Entry> {
        // Naively assumes from is local path
        // FIXME: check for remote paths as well
        // and come up with way to resolve local path from remote
        let from = Self::canonicalize_path(from);
        self.0.get(&from)
    }

    fn canonicalize_path(local_path: &str) -> String {
        fs::canonicalize(local_path)
            .expect("File Exists")
            .into_os_string()
            .into_string()
            .unwrap()
    }

    pub fn insert(&mut self, mut entry: Entry) {
        entry.local_path = Self::canonicalize_path(entry.local_path.as_str());
        self.0.insert(entry.local_path.clone(), entry);
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
        let config: _Config = match toml::from_str(contents) {
            Ok(config) => config,
            // FIXME: return error on config error, default on empty
            Err(_) => Default::default(),
        };
        return config.into_config().unwrap();
        // return config;
    }

    pub fn _write(contents: &str) {
        File::create(DB_PATH)
            .expect("db can be opened")
            .write_all(contents.as_bytes())
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
        let mut doc = contents
            .parse::<toml_edit::Document>()
            .expect("Invalid Config");

        doc["mappings"][entry.local_path.as_str()] = toml_edit::value(entry.remote_path.as_str());

        Self::_write(doc.to_string().as_str());

        // update self with this change as well
        //
        // doubles as an excuse to make self mut like it should be
        // without clippy yelling at me for making it mut when it
        // doesn't have to be
        self.mappings.insert(entry);
    }
}

/// Used to avoid positional args
#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct Entry {
    pub remote_path: String,
    #[serde(skip)]
    pub local_path: String,
}

impl Entry {
    pub fn new(local_path: String, remote_path: String) -> Self {
        Self {
            remote_path,
            local_path,
        }
    }
}

impl TryFrom<(String, toml::Value)> for Entry {
    type Error = anyhow::Error;

    fn try_from((local_path, val): (String, toml::Value)) -> Result<Self, Self::Error> {
        match val {
            toml::Value::String(str) => Ok(Self::new(local_path, str)),
            toml::Value::Table(table) => {
                let mut entry = Self::try_from(table)?;
                entry.local_path = local_path;
                Ok(entry)
            },
            _ => bail!("Allowed Mapping Values are Options Array (Table) and Remote Path (String)"),
        }
    }
}

impl TryFrom<toml::value::Table> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: toml::value::Table) -> Result<Self, Self::Error> {
        toml::from_str(toml::to_string(&value)?.as_str()).with_context(|| "Welp")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_string_mapping() {
        let toml = r#"
[mappings]
"/dev/null" = "dropbox:"
"#;
        let config = Config::_load(toml);
        let entry = Entry {
            local_path: "/dev/null".to_string(),
            remote_path: "dropbox:".to_string(),
        };
        let mappings: HashMap<String, Entry> =
            [(entry.local_path.clone(), entry)].into_iter().collect();
        assert!(config.mappings.0 == mappings, "{:?}", config.mappings);
    }

    #[test]
    fn load_table_mapping() {
        let toml = r#"
[mappings]
"/dev/null" = {remote_path="dropbox:"}
"#;
        let config = Config::_load(toml);
        let entry = Entry {
            local_path: "/dev/null".to_string(),
            remote_path: "dropbox:".to_string(),
        };
        let mappings: HashMap<String, Entry> =
            [(entry.local_path.clone(), entry)].into_iter().collect();
        assert!(config.mappings.0 == mappings, "{:?}", config.mappings);
    }
}
