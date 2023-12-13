#![allow(unused)]

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

    pub fn generate(&self) -> Result<String> {
        Ok(format!(
            "ssh {}@{} {} {}",
            self.user()?,
            self.address()?,
            self.key()?,
            self.jump_host()?
        ))
    }

    fn jump_host(&self) -> Result<String> {
        Ok(String::new())
    }

    fn key(&self) -> Result<String> {
        Ok(String::new())
    }

    fn address(&self) -> Result<String> {
        let address_type = self
            .opts
            .address_type
            .clone()
            .or(self.config.address_type.clone());

        match address_type {
            Some(address_type) => match address_type.as_str() {
                "public" => Ok(self.instance.public_ip.clone().unwrap()),
                "private" => Ok(self.instance.private_ip.clone().unwrap()),
                _ => Err(anyhow::anyhow!("Invalid address type")),
            },
            None => Ok(self.instance.private_ip.clone().unwrap()),
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
