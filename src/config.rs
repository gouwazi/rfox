use std::io::{Error, ErrorKind};
use std::{io::Read, time::Duration};

use std::fs::File;

use log::{error, info};
use serde::Deserialize;

use tokio::io;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub enum IngressMode {
    Http,
    Https,
    SocksV4,
    SocksV5,
}

#[derive(Deserialize, Clone, Debug)]
pub struct IngressConfig {
    pub mode: IngressMode,
    pub bind_address: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct RfoxConfig {
    pub ingress: Vec<IngressConfig>,
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

impl std::fmt::Debug for RfoxConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RfoxConfig")
            .field("ingress", &self.ingress)
            .finish()
    }
}
