use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize, Debug, Clone)]
pub struct Source {
    pub id: String,
    pub name: String,
    pub path: String,
    pub resolution: usize,
    pub link: Option<String>,
    pub attributions: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub cache_size: usize,
    pub sources: Vec<Source>,
}

/// Returns new config with default values
pub fn new() -> Config {
    Config {
        host: String::from("127.0.0.1"),
        port: 50051,
        cache_size: 10_000_000,
        sources: Vec::with_capacity(0),
    }
}

impl Config {
    // load_from load config toml from file "source"
    pub fn load_from(&mut self, source: &Path) -> Result<(), &str> {
        // Read index file content
        let file_content = match fs::read_to_string(source) {
            Ok(v) => v,
            Err(_e) => return Err("error"),
        };

        // Read TOML
        *self = match toml::from_str(&file_content) {
            Ok(v) => v,
            Err(_e) => return Err("error"),
        };

        Ok(())
    }

    pub fn host(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
