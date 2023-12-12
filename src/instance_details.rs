use anyhow::Result;
use aws_sdk_ec2::types::Instance;
use serde::{Deserialize, Serialize};
use tokio::runtime;

pub const CACHE_FILE: &str = "/tmp/blaze_ssh_cache.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceSet {
    pub instances: Vec<InstanceDetails>,
}

impl InstanceSet {
    pub fn new(instances: Vec<InstanceDetails>) -> Result<Self> {
        Ok(Self { instances })
    }

    fn fetch_cache() -> Result<Self> {
        std::fs::read_to_string(CACHE_FILE)
            .and_then(|cache| serde_json::from_str::<InstanceSet>(&cache).map_err(|e| e.into()))
            .map_err(|e| e.into())
    }

    async fn fetch_remote() -> Result<Self> {
        let aws_config = aws_config::load_from_env().await;
        let aws_client = aws_sdk_ec2::Client::new(&aws_config);
        let response = aws_client.describe_instances().send().await?;
        let instances = response
            .reservations()
            .into_iter()
            .flat_map(|reservation| reservation.instances())
            .map(InstanceDetails::from_instance)
            .flat_map(Result::ok)
            .collect::<Vec<_>>();

        InstanceSet::new(instances)
    }

    pub async fn fetch(cache: bool) -> Result<Self> {
        match cache {
            true => std::fs::read_to_string(CACHE_FILE)
                .and_then(|cache| serde_json::from_str(&cache).map_err(|e| e.into()))
                .or_else(|_| {
                    let rt = runtime::Runtime::new().unwrap();
                    rt.block_on(Self::fetch_remote())
                }),
            false => {
                let rt = runtime::Runtime::new().unwrap();
                rt.block_on(Self::fetch_remote())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

    fn extract_instance_name(instance: &Instance) -> Option<String> {
        instance
            .tags()
            .iter()
            .find(|tag| tag.key == Some("Name".to_string()))
            .cloned()
            .and_then(|tag| tag.value)
    }
}
