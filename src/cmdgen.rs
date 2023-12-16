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
        match self.opts.address_type.clone().unwrap_or_default().as_str() {
            "" => match self.config.address_type.clone() {
                Some(address_type) => match address_type.as_str() {
                    "public" => Ok(self.instance.public_ip.clone().unwrap_or_default()),
                    "private" => Ok(self.instance.private_ip.clone().unwrap_or_default()),
                    _ => Err(anyhow::anyhow!("Invalid address type")),
                },
                None => Ok(self.instance.private_ip.clone().unwrap_or_default()),
            },
            address_type => match address_type {
                "public" => Ok(self.instance.public_ip.clone().unwrap_or_default()),
                "private" => Ok(self.instance.private_ip.clone().unwrap_or_default()),
                _ => Err(anyhow::anyhow!("Invalid address type")),
            },
        }
    }

    fn user(&self) -> Result<String> {
        match self.opts.user.clone().unwrap_or_default().as_str() {
            "" => match self.config.default_user.clone() {
                Some(default_user) => Ok(default_user.to_string()),
                None => Err(anyhow::anyhow!("No username provided")),
            },
            username => Ok(username.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_with_opt_prioritizes_opt() {
        let config = Config {
            default_user: Some(String::from("default-user")),
            private_key: None,
            bastion: None,
            port: None,
            address_type: None,
        };
        let opts = ConnectOptions {
            user: Some(String::from("opt-user")),
            key: None,
            jumphost: None,
            address_type: None,
            port: None,
            search: None,
        };
        let instance = InstanceDetails {
            instance_id: Some(String::from("id")),
            instance_name: Some(String::from("name")),
            public_ip: None,
            private_ip: None,
        };

        let command_generator = CommandGenerator::new(&opts, config, instance).unwrap();
        assert_eq!(command_generator.user().unwrap(), "opt-user");
    }

    #[test]
    fn user_without_opt_uses_config_opt() {
        let config = Config {
            default_user: Some(String::from("default-user")),
            private_key: None,
            bastion: None,
            port: None,
            address_type: None,
        };
        let opts = ConnectOptions {
            user: None,
            key: None,
            jumphost: None,
            address_type: None,
            port: None,
            search: None,
        };

        let instance = InstanceDetails {
            instance_id: Some(String::from("id")),
            instance_name: Some(String::from("name")),
            public_ip: None,
            private_ip: None,
        };

        let command_generator = CommandGenerator::new(&opts, config, instance).unwrap();
        assert_eq!(command_generator.user().unwrap(), "default-user");
    }

    #[test]
    fn address_without_opt_uses_config_opt() {
        let config = Config {
            default_user: None,
            private_key: None,
            bastion: None,
            port: None,
            address_type: Some(String::from("public")),
        };
        let opts = ConnectOptions {
            user: None,
            key: None,
            jumphost: None,
            address_type: None,
            port: None,
            search: None,
        };
        let instance = InstanceDetails {
            instance_id: Some(String::from("id")),
            instance_name: Some(String::from("name")),
            public_ip: Some(String::from("public-ip")),
            private_ip: Some(String::from("private-ip")),
        };

        let command_generator = CommandGenerator::new(&opts, config, instance).unwrap();
        assert_eq!(command_generator.address().unwrap(), "public-ip");
    }

    #[test]
    fn address_with_opt_prioritizes_opt() {
        let config = Config {
            default_user: None,
            private_key: None,
            bastion: None,
            port: None,
            address_type: None,
        };
        let opts = ConnectOptions {
            user: None,
            key: None,
            jumphost: None,
            address_type: Some(String::from("public")),
            port: None,
            search: None,
        };
        let instance = InstanceDetails {
            instance_id: Some(String::from("id")),
            instance_name: Some(String::from("name")),
            public_ip: Some(String::from("public-ip")),
            private_ip: Some(String::from("private-ip")),
        };

        let command_generator = CommandGenerator::new(&opts, config, instance).unwrap();
        assert_eq!(command_generator.address().unwrap(), "public-ip");
    }
}
