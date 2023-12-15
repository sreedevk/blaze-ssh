use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const CONFIG_PATH: &str = "~/.config/blaze/config.toml";
#[allow(dead_code)]
pub const DEFAULT: &str = r#"
# Rename this section to "config" to use this as your default config
[config.sample]
    private-key = ""
    default-user = "ec2-user"
    bastion = ""
    port = 22
    address-type = "private"
"#;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(rename = "default-user")]
    pub default_user: Option<String>,
    #[serde(rename = "private-key")]
    pub private_key: Option<PathBuf>,
    pub bastion: Option<String>,
    pub port: Option<u16>,
    #[serde(rename = "address-type")]
    pub address_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ConfigFile {
    config: Config,
}

impl Config {
    pub fn get_config_path(path: Option<PathBuf>) -> Result<String> {
        Ok(path
            .and_then(|cfg_path| Some(String::from(cfg_path.to_str()?)))
            .unwrap_or(CONFIG_PATH.to_string()))
    }

    #[allow(dead_code)]
    pub fn generate_default_config() -> Result<()> {
        std::fs::write(
            PathBuf::from(shellexpand::tilde(CONFIG_PATH).to_string()),
            DEFAULT,
        )?;

        Ok(())
    }

    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        let config_path =
            PathBuf::from(shellexpand::tilde(&Self::get_config_path(path)?).to_string());
        let raw_config = std::fs::read_to_string(config_path);
        match raw_config {
            Ok(config) => Ok(toml::from_str::<ConfigFile>(&config)?.config),
            Err(_e) => Err(anyhow!("Config not found at {}", CONFIG_PATH)),
        }
    }
}
