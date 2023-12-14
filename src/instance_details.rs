use anyhow::Result;
use aws_sdk_ec2::types::Instance;
use serde::{Deserialize, Serialize};

use crate::opts::Opts;

pub const CACHE_FILE: &str = "/tmp/blaze_ssh_cache.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceSet {
    pub instances: Vec<InstanceDetails>,
}

impl InstanceSet {
    pub fn new(instances: Vec<InstanceDetails>) -> Result<Self> {
        Ok(Self { instances })
    }

    async fn fetch_remote() -> Result<Self> {
        let aws_config = aws_config::load_from_env().await;
        let aws_client = aws_sdk_ec2::Client::new(&aws_config);
        let response = aws_client.describe_instances().send().await?;
        let instances = response
            .reservations()
            .iter()
            .flat_map(|reservation| reservation.instances())
            .map(InstanceDetails::from_instance)
            .flat_map(Result::ok)
            .collect::<Vec<_>>();

        let instance_set = InstanceSet::new(instances)?;
        instance_set.write()?;

        Ok(instance_set)
    }

    pub async fn fetch(opts: &Opts) -> Result<Self> {
        match opts.no_cache {
            false => {
                let cache_result = std::fs::read_to_string(CACHE_FILE);
                match cache_result {
                    Ok(cache) => serde_json::from_str(&cache).map_err(|e| e.into()),
                    Err(_e) => Self::fetch_remote().await,
                }
            }
            true => Self::fetch_remote().await,
        }
    }

    pub fn write(&self) -> Result<()> {
        std::fs::write(CACHE_FILE, serde_json::to_string(self)?)?;

        Ok(())
    }

    pub fn filter(&self, search: &str) -> Result<Self> {
        let mut instances = self.instances.clone();
        instances.retain(|instance| {
            instance
                .instance_name
                .clone()
                .unwrap_or("".to_string())
                .contains(search)
        });

        Self::new(instances)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct InstanceDetails {
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub instance_id: Option<String>,
    pub instance_name: Option<String>,
}

impl InstanceDetails {
    pub fn from_instance(instance: &Instance) -> Result<Self> {
        let instance_clone = instance.clone();
        Ok(InstanceDetails {
            public_ip: instance_clone.public_ip_address,
            private_ip: instance_clone.private_ip_address,
            instance_id: instance_clone.instance_id,
            instance_name: Self::extract_instance_name(&instance.clone()),
        })
    }

    pub fn is_empty(&self) -> bool {
        self.public_ip.is_none()
            && self.private_ip.is_none()
            && self.instance_id.is_none()
            && self.instance_name.is_none()
    }

    pub fn display_name(&self) -> Result<String> {
        let cloned_instance = self.clone();
        Ok(format!(
            "{:<32} | priv_ip: {:12} | pub_ip: {:>12} | {:<32}",
            cloned_instance.instance_name.unwrap_or("None".to_string()),
            cloned_instance.private_ip.unwrap_or("None".to_string()),
            cloned_instance.public_ip.unwrap_or("None".to_string()),
            cloned_instance.instance_id.unwrap_or("None".to_string())
        ))
    }

    fn extract_instance_name(instance: &Instance) -> Option<String> {
        instance
            .tags()
            .iter()
            .find(|tag| tag.key == Some("Name".to_string()))
            .cloned()
            .and_then(|tag| tag.value)
    }
}
