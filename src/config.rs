use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub thread_count: usize,
    pub host: String,
    pub port: u16,
    pub directory: String,
    pub tls: Option<Tls>,
}

/// TLS config
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tls {
    pub cert: String,
    pub key: String,
}

impl TryFrom<&str> for Config {
    type Error = Error;

    fn try_from(value: &str) -> Result<Config, Self::Error> {
        let content = fs::read_to_string(value).unwrap();
        let cfg: Config = serde_yaml::from_str(&content).unwrap();
        Ok(cfg)
    }
}
