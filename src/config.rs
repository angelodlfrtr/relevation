use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
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
    pub host: Option<String>,
    pub port: Option<u16>,
    pub cache_size: Option<usize>,
    pub sources: Option<Vec<Source>>,
}

// load_from load config toml from file "source"
pub fn new() -> Config {
    Config {
        host: Some(String::from("127.0.0.1")),
        port: Some(50051),
        cache_size: Some(10_000_000),
        sources: Some(Vec::with_capacity(0)),
    }
}

impl Config {
    pub fn load_from(&mut self, source: &PathBuf) -> Result<(), &str> {
        // Read index file content
        let file_content = match fs::read_to_string(source.as_path()) {
            Ok(v) => v,
            Err(_e) => return Err("error"),
        };

        // Read TOML
        *self = match toml::from_str(&file_content.to_string()) {
            Ok(v) => v,
            Err(_e) => return Err("error"),
        };

        // Set default value
        if self.host.is_none() {
            self.host = Some("127.0.0.1".to_string());
        }
        if self.port.is_none() {
            self.port = Some(50051);
        }

        Ok(())
    }

    pub fn host(&self) -> String {
        format!(
            "{}:{}",
            self.host.clone().unwrap(),
            self.port.clone().unwrap()
        )
    }
}
