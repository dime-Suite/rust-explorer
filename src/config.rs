use anyhow::Ok;
use log::{info, warn};
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
    pub logger: LoggerConfig,
    pub db: DbConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
    // pub tls: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggerConfig {
    pub enable_level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub is_sqlite: bool,
    pub db_uri: String,
    pub max_connections: u32,
}

impl Config {
    /// Parse configuration file.
    pub fn parse(path: &str) -> anyhow::Result<Self> {
        let config_str = fs::read_to_string(path)?;

        let config = toml::from_str(&config_str)?;

        Ok(config)
    }

    pub fn create_or_load_config(path: &str) -> anyhow::Result<Self> {
        if !Path::new(&path).exists() {
            let default_config_contents: &[u8] = include_bytes!("../defaultConfig.toml");
            warn!("Configuration file doesn't exists.");
            let mut file =
                File::create(path).expect("notrace - Failed to create configuration file");
            file.write_all(default_config_contents)
                .expect("notrace - Failed to create configuration file");
            info!("Created configuration file. Exiting...");
            // std::process::exit(0);
        }
        Ok(Config::parse(&path).expect("notrace - Failed to parse configuration file"))
    }
}
