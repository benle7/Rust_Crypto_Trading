use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub ip_port: String,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub schema_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

pub fn load_config(file_path: &str) -> Config {
    let config = fs::read_to_string(file_path)
        .expect(format!("Failed to read config file: {}", file_path).as_str());
    serde_yaml::from_str(&config)
        .expect(format!("Failed to parse config file: {}", file_path).as_str())
}
