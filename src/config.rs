use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const CONFIG_PATH: &str = "~/.config/blaze/config.toml";

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(rename = "default-user")]
    pub default_user: Option<String>,
    #[serde(rename = "private-key")]
    pub private_key: Option<PathBuf>,
    pub bastion: Option<String>,
    pub port: Option<u16>,
    #[serde(rename = "address-type")]
    pub address_type: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct ConfigFile {
    config: Config,
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        let config_path_string = path
            .and_then(|cfg_path| Some(String::from(cfg_path.to_str()?)))
            .unwrap_or(CONFIG_PATH.to_string());

        let config_path = PathBuf::from(shellexpand::tilde(&config_path_string).to_string());
        let raw_config = std::fs::read_to_string(config_path)?;

        Ok(toml::from_str::<ConfigFile>(&raw_config)?.config)
    }
}

// TODO: GENERATE DEFAULT CONFIG IF DOES NOT EXIST
