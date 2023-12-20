use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const CONFIG_PATH: &str = "~/.config/blaze/config.toml";
#[allow(dead_code)]
pub const DEFAULT: &str = r#"
[config]
    private-key = ""
    default-user = "ec2-user"
    jumphost = ""
    port = 22
    address-type = "private"
"#;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(rename = "default-user")]
    pub default_user: Option<String>,
    #[serde(rename = "private-key")]
    pub private_key: Option<PathBuf>,
    pub jumphost: Option<String>,
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

    pub fn write_default_config() -> Result<()> {
        let expanded_path = PathBuf::from(shellexpand::tilde(CONFIG_PATH).to_string());
        match expanded_path.parent() {
            Some(parent) => {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }

                std::fs::write(
                    PathBuf::from(shellexpand::tilde(CONFIG_PATH).to_string()),
                    DEFAULT,
                )?;
            }
            None => {
                return Err(anyhow!("Invalid config path"));
            }
        }

        Ok(())
    }

    pub fn read_raw(path: Option<PathBuf>) -> Result<String> {
        let config_path =
            PathBuf::from(shellexpand::tilde(&Self::get_config_path(path)?).to_string());

        std::fs::read_to_string(config_path).map_err(|e| e.into())
    }

    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        let config_path =
            PathBuf::from(shellexpand::tilde(&Self::get_config_path(path)?).to_string());
        let raw_config = std::fs::read_to_string(config_path);
        match raw_config {
            Ok(config) => Ok(toml::from_str::<ConfigFile>(&config)?.config),
            Err(_e) => Err(anyhow!("Config not found at {}, Please use `blssh configure` to generate default configuration.", CONFIG_PATH)),
        }
    }
}
