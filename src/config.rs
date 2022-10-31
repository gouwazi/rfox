use std::io::{Error, ErrorKind};
use std::{time::Duration, io::Read};

use std::fs::File;

use serde::Deserialize;
use log::{error, info};

use tokio::io;

#[derive(Deserialize, Clone, PartialEq)]
pub enum IngressMode {
    Http,
    Https,
    SocksV4,
    SocksV5,
}

#[derive(Deserialize, Clone)]
pub struct IngressConfig {
    pub mode: IngressMode,
    pub bind_address: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct RfoxConfig {
    pub ingress: Vec<IngressConfig>
}

impl RfoxConfig {
    pub fn read_from_config(filename: &str) -> io::Result<RfoxConfig> {
        let mut file = File::open(filename).map_err(|e| {
            error!("Error opening config file {}: {}", filename, e);
            e
        })?;

        let mut yaml = vec![];

        file.read_to_end(&mut yaml).map_err(|e| {
            error!("Error reading file {}: {}", filename, e);
            e
        })?;

        let result: RfoxConfig = serde_yaml::from_slice(&yaml).map_err(|e| {
            error!("Error parsing yaml {}: {}", filename, e);
            Error::from(ErrorKind::InvalidInput)
        })?;

        Ok(result)
    }
}
