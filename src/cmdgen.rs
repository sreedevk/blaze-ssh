use std::process::Command;

use crate::instance_details::InstanceDetails;
use crate::{config::Config, opts::ConnectOptions};

use anyhow::Result;

pub struct CommandGenerator {
    opts: ConnectOptions,
    config: Config,
    instance: InstanceDetails,
}

impl CommandGenerator {
    pub fn new(opts: &ConnectOptions, config: Config, instance: InstanceDetails) -> Result<Self> {
        Ok(Self {
            opts: opts.clone(),
            config,
            instance,
        })
    }

    pub fn generate<'a>(&self, cmd: &'a mut Command) -> Result<&'a mut Command> {
        let ssh_command = vec![
            String::from("ssh -t"),
            format!("{}@{}", self.user()?, self.address()?),
            self.key()?,
            self.jump_host()?,
        ]
        .into_iter()
        .filter(|arg| !arg.is_empty())
        .collect::<Vec<String>>()
        .join(" ");

        Ok(cmd
            .arg("-c")
            .arg(ssh_command)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit()))
    }

    fn jump_host(&self) -> Result<String> {
        match self.opts.jumphost.clone() {
            Some(jumphost) => Ok(format!("-J {}", jumphost)),
            None => Ok(String::new()),
        }
    }

    fn key(&self) -> Result<String> {
        let key = self.opts.key.clone().or(self.config.private_key.clone());

        match key {
            Some(key) => Ok(format!("-i {}", shellexpand::tilde(key.to_str().unwrap()))),
            None => Ok(String::new()),
        }
    }

    fn address(&self) -> Result<String> {
        let address_type = self
            .opts
            .address_type
            .clone()
            .or(self.config.address_type.clone());

        match address_type {
            Some(address_type) => match address_type.as_str() {
                "public" => Ok(self.instance.public_ip.clone().unwrap_or_default()),
                "private" => Ok(self.instance.private_ip.clone().unwrap_or_default()),
                _ => Err(anyhow::anyhow!("Invalid address type")),
            },
            None => Ok(self.instance.private_ip.clone().unwrap_or_default()),
        }
    }

    fn user(&self) -> Result<String> {
        let username = self
            .opts
            .user
            .clone()
            .or(Some(self.config.default_user.clone().unwrap()));

        match username {
            Some(username) => Ok(username),
            None => Err(anyhow::anyhow!("No username provided")),
        }
    }
}
